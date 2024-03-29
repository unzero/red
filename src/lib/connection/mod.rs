mod ssh_connection;

use actix_web::Result;
use ssh_connection::SshConnection;

#[derive(Debug)]
pub struct ConnectionError{
    msg: String
}

impl ConnectionError {
    pub fn new(e: &str) -> Self {
        Self { msg: e.into() }
    }
    
    pub fn default_error() -> Self {
        Self { msg: "Could not create the ssh session.".into() }
    }
}

pub trait Connection {
    fn execute(&self, cmd: &str) -> Result<String, ConnectionError>;
    fn check_connection(&self) -> Result<(), ConnectionError>;
}

pub fn get_ssh_connection(host: &str, port: &str, username: &str, password: &str) -> Result<Box<dyn Connection>, ConnectionError> {
    Ok(Box::new(SshConnection::new(host, port, username, password)?))
}
