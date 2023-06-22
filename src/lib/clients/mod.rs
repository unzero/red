use std::{sync::{Arc, Mutex}, collections::HashMap};

#[derive(Clone, Debug)]
pub struct RedUser {
    pub host: String, 
    pub user: String, 
    pub pass: String,
}

impl RedUser {
    pub fn new(host: String, user: String, pass: String ) -> Self {
        Self { host, user, pass }
    }
}

pub type RedUsers = Arc<Mutex<HashMap<String, RedUser>>>;
