pub mod connection;

#[cfg(test)]
mod tests {
    use crate::connection::{send, ConnectionPool};
    use super::*;

    #[test]
    fn it_works() {
        let pool = ConnectionPool::new("localhost:6381", 1);
        let mut socket = pool.get().unwrap();

        let res = send(&mut socket, "ls\'*\'\0".as_bytes()).expect("panic message");

        let s = String::from_utf8_lossy(&res);
        println!("{}", s);

        pool.put(socket);
        pool.close();
    }
}
