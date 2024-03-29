/* This module holds the logic for a client. 
    A client knows what to do with a connection, 
    we need here concrete operations, read_file, change_directory, etc.
 */

pub mod ssh_client;

struct ClientError;

pub trait Client {
    fn execute(&mut self, cmd :&str) -> String;
}
