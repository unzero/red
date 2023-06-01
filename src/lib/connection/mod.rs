use std::net::TcpStream; 
use ssh2::Session;

pub fn check_connection(host: & str, port: i32, user: & str, password: & str) -> bool{
    let hostname = format!("{}:{}", host, port);
    let tcp = TcpStream::connect(hostname).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    match sess.userauth_password(user, password){
        Ok(_) => true,
        _ => false,
    }
}


