/* This files holds the logic for a ssh client.
*/

use crate::lib::user::{Client, UserError};

#[derive(Debug)]
pub struct SshUser {
    pub host: String, 
    pub username: String, 
    pub password: String,
    current_path: String,
}

impl SshUser {
    pub fn new(host: String, username: String, password: String) -> Self {
        let current_path = "/home".to_string();
        Self { host, username, password, current_path }
    }
}

impl Client for SshUser {
    fn execute(&self, cmd :&str) -> Result<String, UserError> {
        Ok("ok".into())
    }

    fn check_connection(&self) -> Result<(), UserError> {
        Ok(())
    }
}
