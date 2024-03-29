pub mod ssh_connection;

struct ConnectionError;

trait Connection {
    fn execute(&self, cmd: &str) -> Result<String, ConnectionError>;
}