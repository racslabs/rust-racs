pub mod socket;
pub mod pack;
mod frame;
mod command;
mod utils;

#[cfg(test)]
mod tests {
    use crate::socket::{send, SocketPool};
    use crate::command::Command;

    #[test]
    fn it_works() {
        let pool = SocketPool::new("127.0.0.1:6381", 1);
        let command = Command::new(pool);
        let r = command.execute_command("ls '*'");
        
        println!("{:?}", r);
    }
}
