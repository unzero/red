
#[derive(Debug)]
pub enum RedError {
    ConnectionError, 
    UserError,
    ClientError,
    FileError,
    OtherError(&'static str),
}
