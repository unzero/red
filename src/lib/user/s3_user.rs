
use crate::lib::user::{Client, UserError};

pub struct S3User;

impl S3User{
    pub fn new() -> Self {
        Self {}
    }
}

impl Client for S3User {
    fn check_connection(&self) -> Result<(), UserError> {
        Err(UserError::new("We cannot support S3 yet.".into()))
    }

    fn execute(&self, cmd: &str) -> Result<String, UserError> {
        Err(UserError::new("We cannot support S3 yet.".into()))
    }
}

