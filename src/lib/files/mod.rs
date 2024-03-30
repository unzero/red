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

pub fn get_file_type(filename: String) -> String {
    let ext = std::path::Path::new(&filename).
                extension().and_then(std::ffi::OsStr::to_str).unwrap_or("text");
    match ext { 
        "py" => String::from("python"),
        "rs" => String::from("rust"),
        "js" => String::from("javascript"),
        _ => String::from(ext)
    }
}
