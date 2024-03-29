use std::collections::HashMap;

use serde::de::IntoDeserializer;

use super::{Client, UserError};

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

    fn check_connection(&self) -> Result<(), UserError> {
        Err(UserError::new("We cannot support S3 yet.".into()))
    }

    fn execute(&self, cmd: &str) -> Result<String, UserError> {
        Err(UserError::new("We cannot support S3 yet.".into()))
    }

    fn get_files(&mut self) -> Result<Vec<HashMap<String, String>>, UserError> {
        Err(UserError::new("We cannot support S3 yet.".into()))
    }
}
