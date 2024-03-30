mod ssh_connection;

use ssh_connection::SshConnection;
use crate::lib::errors::RedError;

pub trait Connection {
    fn execute(&self, cmd: &str) -> Result<String, RedError>;
    fn check_connection(&self) -> Result<(), RedError>;
    fn read_file_content(&self, filepath: &str) -> Result<String, RedError>;
    fn save_file(&self, filepath: &str, file_content: &str) -> Result<String, RedError>;
}

pub fn get_ssh_connection(host: &str, port: &str, username: &str, password: &str) -> Result<Box<dyn Connection>, RedError> {
    Ok(Box::new(SshConnection::new(host, port, username, password)?))
}
