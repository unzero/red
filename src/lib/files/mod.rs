use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Redfile {
    file_uuid: String,
}

impl Redfile {
    pub fn get_uuid(&self) -> String {
        self.file_uuid.clone()
    }
}

