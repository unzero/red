use std::{sync::{Arc, Mutex}, collections::HashMap, string::String};

use crate::lib::user::Client;

pub type RedUsers = Arc<Mutex<HashMap<String, Box<dyn Client + Send> >>>;
