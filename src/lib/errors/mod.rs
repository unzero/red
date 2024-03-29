
pub enum RedError {
    ConnectionError, 
    UserError,
    ClientError,
    OtherError(&'static str),
}
