use std::net::TcpStream; 
use ssh2::Session;

pub fn check_connection(host: &str, port: i32, user: &str, password: &str) -> bool{
    let hostname = format!("{}:{}", host, port);
    let tcp = match TcpStream::connect(hostname) {
        Ok(x) => x,
        _ => return false, 
    };
    let mut sess = match Session::new(){
        Ok(x) => x, 
        _ => return false,
    };
    sess.set_tcp_stream(tcp);
    match sess.handshake(){
        Ok(_) => (),
        _ => return false,
    };
    match sess.userauth_password(user, password){
        Ok(_) => true,
        _ => false,
    }
}


