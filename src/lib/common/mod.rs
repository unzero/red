use std::{sync::{Arc, Mutex}, collections::HashMap, string::String};
use serde::{Deserialize, Serialize};

use crate::lib::user::Client;

pub type RedUsers = Arc<Mutex<HashMap<String, Box<dyn Client + Send> >>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct RedLogin{
    pub host: String,
    pub username: String,
    pub password: String,
}
