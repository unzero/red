
use crate::lib::connection::{Connection, ConnectionError};

struct SshConnection;

impl Connection for SshConnection {
    fn execute(&self, cmd: &str) -> Result<String, ConnectionError>{
        Ok("ok".into())
    }
}
