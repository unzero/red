use std::net::TcpStream;
use ssh2::Session;
use std::{io::prelude::*, path::Path};

use super::Connection;
use crate::lib::errors::RedError;

pub struct SshConnection{
    session: Session
}

impl SshConnection {
    pub fn new(host: &str, port: &str, username: &str, password: &str) -> Result<Self, RedError> {
        let hostname = format!("{}:{}", host, port);
        let tcp = TcpStream::connect(hostname).map_err( |_e| RedError::ConnectionError )?;
        let mut sess = Session::new().map_err( |_e| RedError::ConnectionError )?;
        sess.set_tcp_stream(tcp);
        sess.handshake().map_err( |_e| RedError::ConnectionError )?;
        sess.userauth_password(username, password).map_err( |_e| RedError::ConnectionError )?;
        Ok(Self{ session: sess })
    }
}

impl Connection for SshConnection {
    fn execute(&self, cmd: &str) -> Result<String, RedError>{
        let mut channel = self.session.channel_session().map_err(|_e| RedError::ConnectionError )?;
        let mut result = String::new();
        channel.exec(cmd).map_err( |_e| RedError::ConnectionError )?;
        channel.read_to_string(&mut result).map_err( |_e| RedError::ConnectionError )?;
        channel.wait_close().map_err( |_e| RedError::ConnectionError )?;
        Ok(result)
    }

    fn check_connection(&self) -> Result<(), RedError> {
        Ok(())
    }
}
