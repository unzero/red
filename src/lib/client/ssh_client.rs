

use crate::lib::client::{Client, ssh_user::SshUser}
use crate::lib::connection::ssh_connection::SshConnection;

pub struct SshClient {
    user : SshUser,
    connection: SshConnection,
}

impl Client for SshClient{

    pub fn new(host: String, user: String, pass: String ) -> Self {
        let current_path = connection::get_initial_path(
                connection::SshInformation::new(host.clone(), SSH_PORT, user.clone(), pass.clone())
            ).unwrap().replace('\n', "");
        let available_files = HashMap::new();
        Self { host, user, pass, current_path, available_files}
    }

    fn execute(&mut self, cmd :&str) -> String {
        "Ok".into()
    }
}