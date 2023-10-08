use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Redfile {
    file_uuid: String,
    file_content: Option<String>,
}

impl Redfile {
    pub fn get_uuid(&self) -> String {
        self.file_uuid.clone()
    }

    pub fn get_file_content(&self) -> Option<String>{
        self.file_content.clone()
    }
}

