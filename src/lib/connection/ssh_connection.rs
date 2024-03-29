use std::net::TcpStream;
use ssh2::Session;
use std::{io::prelude::*, path::Path};

use super::{Connection, ConnectionError};

pub struct SshConnection{
    session: Session
}

impl SshConnection {
    pub fn new(host: &str, port: &str, username: &str, password: &str) -> Result<Self, ConnectionError> {
        let hostname = format!("{}:{}", host, port);
        let tcp = TcpStream::connect(hostname).map_err( |_e| ConnectionError::default_error() )?;
        let mut sess = Session::new().map_err( |_e| ConnectionError::default_error() )?;
        sess.set_tcp_stream(tcp);
        sess.handshake().map_err( |_e| ConnectionError::default_error() )?;
        sess.userauth_password(username, password).map_err( |_e| ConnectionError::default_error() )?;
        Ok(Self{ session: sess })
    }
}

impl Connection for SshConnection {
    fn execute(&self, cmd: &str) -> Result<String, ConnectionError>{
        let mut channel = self.session.channel_session().map_err(|_e| ConnectionError::default_error() )?;
        let mut result = String::new();
        channel.exec(cmd).map_err( |_e| ConnectionError::default_error() )?;
        channel.read_to_string(&mut result).map_err( |_e| ConnectionError::default_error() )?;
        channel.wait_close().map_err( |_e| ConnectionError::default_error() )?;
        Ok(result)
    }

    fn check_connection(&self) -> Result<(), ConnectionError> {
        Ok(())
    }
}
