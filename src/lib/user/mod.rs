/* This module stores information about a concrete client. 
    It represents the minimal information for multiple data source, s3, ssh, etc.
*/

mod ssh_user;
mod s3_user;

use std::{sync::{Arc, Mutex}, collections::HashMap, string::String};

use crate::web::red::RedLogin;

use self::ssh_user::SshUser;
use self::s3_user::S3User;

pub type RedUsers = Arc<Mutex<HashMap<String, Box<dyn Client + Send> >>>;

pub struct UserError {
    error : String
}

impl UserError {
    pub fn new(e: String) -> Self { 
        Self { error: e }
    }
}

pub trait Client {
    fn get_host(&self) -> String;
    fn get_username(&self) -> String;
    fn execute(&self, cmd: &str) -> Result<String, UserError>;
    fn check_connection(&self) -> Result<(), UserError>;
    fn get_files(&mut self) -> Result<Vec<HashMap<String, String>>, UserError>;
}

pub fn new_client(kind: &str, client_data: RedLogin) -> Result<Box<dyn Client + Send>, UserError> {
    let user: Box<dyn Client + Send>;
    match kind {
        "ssh" => {
            user = Box::new(SshUser::new(client_data.host, client_data.username, client_data.password));
        }
        _ => {
            user = Box::new(S3User::new());
        }
    }
    user.check_connection()?;
    Ok(user)
}
