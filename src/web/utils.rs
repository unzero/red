use std::{collections::HashMap, string::String};

#[macro_export]
macro_rules! context {
    ( $( $x:tt )+ ) => {
        &Context::from_value(serde_json::json!( $($x)+ )).unwrap()
    }
}

#[macro_export]
macro_rules! json_response {
    ( $( $x:tt )+ ) => {
        serde_json::json!( $($x)+ )
    }
}

#[warn(dead_code)]
pub fn get_dummy_files(size: usize) -> Vec<HashMap<String, String>> {
    let mut files = vec![];
    let types = vec!["directory", "file"];
    for i in 0..size {
        files.push( HashMap::from([
            (String::from("name"), String::from("This is a testxxxxxxxxxxxxxxxxxxxxxx")),
            (String::from("type"), String::from(types[i%2])),
            (String::from("uuid"), "0x0000".to_string()),
            ]));
    }
    files
}

pub fn get_templates_route () -> tera::Tera {
    tera::Tera::new("src/templates/**/*.html").unwrap()   
}
