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
    available_files: HashMap<String, String>, 
}

impl RedUser {
    pub fn new(host: String, user: String, pass: String ) -> Self {
        let current_path = connection::get_initial_path(
                connection::SshInformation::new(host.clone(), SSH_PORT, user.clone(), pass.clone())
            ).unwrap().replace('\n', "");
        let available_files = HashMap::new();
        Self { host, user, pass, current_path, available_files}
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
        print!("execute_cd cmd {}\n", cmd);
        self.current_path = partial_result.replace("\n", "");
        print!("execute_cd self.current_path {}\n", self.current_path);
        "Ok".to_string()
    }

    pub fn execute_file(&mut self) -> Vec<HashMap<String, String>> {
        let cmd = format!("cd {}; file *", self.current_path);
        print!("execute_file cmd {}\n", cmd);
        let partial_result = self.execute_cmd(&cmd.to_owned());
        print!("execute_file partial_result {}\n", partial_result);
        //Update the available_files table
        self.available_files = HashMap::from([ (self.get_file_uuid(String::from("..")), String::from("..")) ]);
        let mut files = vec![HashMap::from([
                                           (String::from("name"), String::from("..")),
                                           (String::from("type"), String::from("directory")),
                                           (String::from("uuid"), self.get_file_uuid(String::from(".."))),
        ])];
        //let mut files = vec![vec![String::from(".."), String::from("directory")]];
        if partial_result.contains("No such file or directory") {
            //The directory is emtpy
            return files;
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
        return files
    }

    pub fn read_file_content(&mut self, target: String) -> String {
        match self.available_files.get(&target) { 
            Some(filename) => {
                connection::read_file_content(
                    self.to_ssh_information(), 
                    &self.get_full_path_to(filename.clone()).to_owned()).unwrap()
            }
            _ => String::from("RED ERROR")
        }
    }

    pub fn change_directory(&mut self, target: String) -> Vec<HashMap<String, String>> {
        print!("change_directory target {}\n", target);
        print!("change_directory self.available_files {:?}\n", self.available_files);
        match self.available_files.get(&target) {
            Some(real_target) => {
                self.execute_cd(real_target.clone())
            },
            _ => String::from("None"),
        };
        self.execute_file()
    }

    fn get_full_path_to(&mut self, filename: String) -> String {
        format!("{}/{}", self.current_path, filename)
    }

    fn get_file_uuid(&mut self, filename: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.get_full_path_to(filename).as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn query_file_uuid(&mut self, target: String) -> String {
        match self.available_files.get(&target) {
            Some(filename) => filename.clone(), 
            _ => String::from("RED ERROR"),
        }
    }
}

