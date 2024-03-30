use std::collections::HashMap;

use super::Client;
use crate::lib::errors::RedError;

pub struct S3User;

impl S3User{
    pub fn new() -> Self {
        Self {}
    }
}

impl Client for S3User {
    
    fn get_host(&self) -> String {
        "None".into()
    }

    fn get_username(&self) -> String {
        "None".into()
    }

    fn check_connection(&self) -> Result<(), RedError> {
        Err(RedError::OtherError("We cannot support S3 yet."))
    }

    fn execute(&self, cmd: &str) -> Result<String, RedError> {
        Err(RedError::OtherError("We cannot support S3 yet."))
    }

    fn get_files(&mut self) -> Result<Vec<HashMap<String, String>>, RedError> {
        Err(RedError::OtherError("We cannot support S3 yet."))
    }

    fn query_file_uuid(&mut self, target: String) -> Result<String , RedError> {
        Err(RedError::OtherError("We cannot suport S3 yet."))
    }

    fn read_file_content(&mut self, target: String) -> Result<String, RedError> {
        Err(RedError::OtherError("We cannot suport S3 yet."))
    }

    fn change_directory(&mut self, target: String) -> Result<Vec<HashMap<String, String>>, RedError> {
        Err(RedError::OtherError("We cannot suport S3 yet."))
    }
}
