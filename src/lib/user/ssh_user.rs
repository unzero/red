/* This files holds the logic for a ssh client.
*/
use std::{collections::HashMap, string::String};
use sha2::{Sha256, Digest};
use log;

use super::Client;
use crate::lib::connection::{self, get_ssh_connection};
use crate::lib::errors::RedError;

const SSH_PORT: &str = "22";

#[derive(Debug)]
pub struct SshUser {
    pub host: String, 
    pub username: String, 
    pub password: String,
    current_path: String,
    available_files: HashMap<String, String>,
}

impl SshUser {
    pub fn new(host: String, username: String, password: String) -> Result<Self, RedError> {
        let mut instance = Self { host, username, password, current_path: "/home".into(), available_files: HashMap::new() };
        instance.current_path = instance.execute("pwd")?.replace("\n", "");
        log::debug!("New SshUser with parameters: {:?}", instance);
        Ok(instance)
    }

    fn get_full_path_to(&mut self, filename: String) -> String {
        format!("{}/{}", self.current_path, filename)
    }

    fn get_file_uuid(&mut self, filename: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.get_full_path_to(filename).as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn build_new_connection(&self) -> Result<Box<dyn connection::Connection>, RedError> {
        let conn = get_ssh_connection(self.host.as_str(), SSH_PORT, self.username.as_str(), self.password.as_str())
            .map_err( |_e| RedError::ConnectionError )?;
        Ok(conn)
    }

    pub fn execute_cd(&mut self, location: String) -> Result<(), RedError> {
        let cmd = format!("cd {}; cd {}; pwd", self.current_path, location);
        let partial_result = self.execute(&cmd.to_owned())?;
        self.current_path = partial_result.replace("\n", "");
        Ok(())
    }
}

impl Client for SshUser {
    fn get_host(&self) -> String {
        self.host.clone()
    }

    fn get_username(&self) -> String {
        self.username.clone()
    }

    fn execute(&self, cmd :&str) -> Result<String, RedError> {
        let conn = get_ssh_connection(self.host.as_str(), SSH_PORT, self.username.as_str(), self.password.as_str())
            .map_err( |_e| RedError::ConnectionError )?;
        log::debug!("SshUser trying to execute: {:?}", cmd);
        let result = conn.execute(cmd).map_err(|_e| RedError::ConnectionError )?;
        log::debug!("SshUser executed {:?} with result {:?}", cmd, result);
        Ok(result)
    }

    fn check_connection(&self) -> Result<(), RedError> {
        Ok(())
    }

    fn get_files(&mut self) -> Result<Vec<HashMap<String, String>>, RedError> {
        let cmd = format!("cd {}; file *", self.current_path.as_str());
        let partial_result = self.execute(&cmd).map_err( |_e| RedError::ClientError )?;
        //Update the available_files table
        self.available_files = HashMap::from([ (self.get_file_uuid(String::from("..")), String::from("..")) ]);
        let mut files = vec![HashMap::from([
                                           (String::from("name"), String::from("..")),
                                           (String::from("type"), String::from("directory")),
                                           (String::from("uuid"), self.get_file_uuid(String::from(".."))),
        ])];

        if partial_result.contains("No such file or directory") {
            //The directory is emtpy
            return Ok(files)
        }
        for tmp_file in partial_result.split("\n")
            .filter_map( |s| match String::from(s).len() {
                0 => None,
                _ => Some(String::from(s)) }) {
                let file_info = tmp_file.replace(" ", "").split(":").map(|x| String::from(x)).collect::<Vec<_>>();
                let file_uuid = self.get_file_uuid(file_info[0].clone());
                self.available_files.insert(file_uuid.clone(), file_info[0].clone());
                files.push( HashMap::from([
                                        (String::from("name"), file_info[0].clone()),
                                        (String::from("type"), file_info[1].clone()),
                                        (String::from("uuid"), file_uuid.clone()),
                ]));
            }
        return Ok(files)
    }

    fn query_file_uuid(&mut self, target: String) -> Result<String, RedError> {
        Ok(self.available_files.get(&target).ok_or_else(|| RedError::UserError)?.to_string())
    }

    fn read_file_content(&mut self, target: String) -> Result<String, RedError> {
        let filename = self.available_files.get(&target).ok_or_else(|| RedError::UserError)?;
        let filepath = self.get_full_path_to(filename.into());
        let content = self.build_new_connection()?.read_file_content(filepath.as_str())?;
        Ok(content)
    }

    fn change_directory(&mut self, target: String) -> Result<Vec<HashMap<String, String>>, RedError> {
        let filename = self.available_files.get(&target).ok_or_else(|| RedError::UserError)?;
        self.execute_cd(filename.into())?;
        Ok(self.get_files()?)        
    }

    fn create_new_file(&mut self, target: String) -> Result<String, RedError> {
        let file_uuid = self.get_file_uuid(target.clone());
        let filename = self.available_files.get(&target).ok_or_else(|| RedError::UserError)?;
        if self.available_files.contains_key(filename) {
            return Err(RedError::ClientError);
        }
        let cmd = format!("touch {}", self.get_full_path_to(target.clone()));
        self.execute(&cmd).map_err( |_e| RedError::ClientError )?;
        self.available_files.insert(file_uuid.clone(), target);
        Ok(file_uuid)
    }
}
