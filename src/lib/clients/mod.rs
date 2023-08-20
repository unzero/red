use std::{sync::{Arc, Mutex}, collections::HashMap, string::String};
use crate::lib::connection;
use ssh2::Channel;

const SSH_PORT :i32 = 22;
pub type RedUsers = Arc<Mutex<HashMap<String, RedUser>>>;

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
        print!("Initial path: {}\n", current_path);
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
        print!("\n new path {} {}\n", self.current_path, cmd);
        "Ok".to_string()
    }

    pub fn execute_file(&mut self) -> Vec<String> {
        let cmd = format!("cd {}; file *", self.current_path);
        let partial_result = self.execute_cmd(&cmd.to_owned());
        let files = partial_result.split("\n").map(|s| String::from(s)).collect();

        //TODO: Split and other logic
        files
    }

    pub fn read_file_content(&mut self, filename: String) -> String {
        let filepath = format!("{}/{}", self.current_path, filename);
        connection::read_file_content(
            self.to_ssh_information(), 
            &filepath.to_owned()).unwrap()
    }


}

