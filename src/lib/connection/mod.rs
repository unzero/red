use std::net::TcpStream; 
use ssh2::{Session, Channel};
use std::io::prelude::*;

pub struct SshInformation {
    host: String, 
    port: i32,
    user: String, 
    pass: String,
}

impl SshInformation {
    pub fn new(host: String, port: i32, user: String, pass: String) -> Self {
        Self { host, port, user, pass }
    }
}

fn get_auth_session(user_data: SshInformation) -> Result<Session, &'static str> { 
    let hostname = format!("{}:{}", user_data.host, user_data.port);
    let err_str = "Could not create the ssh session.";
    let tcp = match TcpStream::connect(hostname) {
        Ok(x) => x,
        _ => return Err(&err_str),
    };
    let mut sess = match Session::new(){
        Ok(x) => x, 
        _ => return Err(&err_str),
    };
    sess.set_tcp_stream(tcp);
    match sess.handshake(){
        Ok(_) => (),
        _ => return Err(&err_str),
    };
    match sess.userauth_password(&user_data.user.to_owned(), &user_data.pass.to_owned()){
        Ok(_) => (),
        _ => return Err(&err_str),
    };

    Ok(sess)
}

pub fn get_channel(user_data: SshInformation) -> Result<Channel, &'static str> {
    let sess = match get_auth_session(user_data) {
        Ok(x) => x,
        _ => return Err("Something were wrong!")
    };
    match sess.channel_session() {
        Ok(x) => Ok(x),
        _ => Err("Something were wrong!")
    }
}

pub fn check_connection(user_data: SshInformation) -> bool {
    get_auth_session(user_data).is_ok()
}

pub fn get_initial_path(user_data: SshInformation) -> Result<String, &'static str> {
    let mut channel = get_auth_session(user_data).unwrap().channel_session().unwrap();
    let mut new_path = String::new();
    channel.exec("pwd");
    channel.read_to_string(&mut new_path).unwrap();
    channel.wait_close();
    Ok(new_path)
}

pub fn execute(user_data: SshInformation, cmd: &str) -> Result<String, &'static str> {
    let mut channel = get_auth_session(user_data).unwrap().channel_session().unwrap();
    let mut result = String::new();
    channel.exec(cmd);
    channel.read_to_string(&mut result).unwrap();
    channel.wait_close();
    Ok(result)
}

fn execute_on_shell(channel: &mut Channel, cmd: &str) -> String {
    print!("new call to execute on shell");
    let mut result = String::new();
    channel.write_all(cmd.as_bytes()).unwrap();
    channel.send_eof().unwrap();
    channel.read_to_string(&mut result);
    channel.wait_eof().unwrap();
    print!("new fhs call to execute on shell");
    result
}



