use std::{sync::{Arc, Mutex}, collections::HashMap, string::String};
use crate::lib::connection;
use sha2::{Sha256, Digest};

const SSH_PORT :i32 = 22;
pub type RedUsers = Arc<Mutex<HashMap<String, RedUser>>>;

#[derive(Debug)]
pub struct RedUser {
    pub host: String, 
    pub user: String, 
    pub pass: String,
    current_path: String,
}

impl RedUser {
    pub fn new(host: String, user: String, pass: String ) -> Self {
        let current_path = connection::get_initial_path(
                connection::SshInformation::new(host.clone(), SSH_PORT, user.clone(), pass.clone())
            ).unwrap().replace('\n', "");
        Self { host, user, pass, current_path}
    }

    fn to_ssh_information(&self) -> connection::SshInformation {
        connection::SshInformation::new(self.host.clone(), SSH_PORT, self.user.clone(), self.pass.clone())
    }

    fn execute_cmd(&mut self, cmd_alias: &str) -> String {
        let cmd_run = format!("cd {};{}", self.current_path, cmd_alias);
        let cmd_res = connection::execute(
            self.to_ssh_information(), 
            &cmd_run.to_owned());
        cmd_res.unwrap()
    }

    pub fn execute_cd(&mut self, location: String) -> String {
        let cmd = format!("cd {}; cd {}; pwd", self.current_path, location);
        let partial_result = self.execute_cmd(&cmd.to_owned());
        self.current_path = partial_result.replace("\n", "");
        "Ok".to_string()
    }

    pub fn execute_file(&mut self) -> Vec<Vec<String>> {
        let cmd = format!("cd {}; file *", self.current_path);
        let partial_result = self.execute_cmd(&cmd.to_owned());
        let mut files = vec![vec![String::from(".."), String::from("directory")]];
        if partial_result.contains("No such file or directory") {
            //The directory is emtpy
            return files;
        }
        for tmp_file in partial_result.split("\n")
            .filter_map( |s| match String::from(s).len() {
                0 => None,
                _ => Some(String::from(s)) }) {
            files.push(tmp_file.replace(" ", "").split(":").map(|x| String::from(x)).collect::<Vec<_>>());
        }
        return files
    }

    pub fn read_file_content(&mut self, filename: String) -> String {
        connection::read_file_content(
            self.to_ssh_information(), 
            &self.get_full_path_to(filename).to_owned()).unwrap()
    }

    pub fn change_directory(&mut self, target: String) -> Vec<Vec<String>> {
        self.execute_cd(target);
        self.execute_file()
    }

    fn get_full_path_to(&mut self, filename: String) -> String {
        format!("{}/{}", self.current_path, filename)
    }

    pub fn get_file_uuid(&mut self, filename: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.get_full_path_to(filename).as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

