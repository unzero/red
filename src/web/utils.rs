#[macro_export]
macro_rules! context {
    ( $( $x:tt )+ ) => {
        &Context::from_value(serde_json::json!( $($x)+ )).unwrap()
    }
}
