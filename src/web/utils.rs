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
