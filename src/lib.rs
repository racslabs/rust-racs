pub mod socket;
pub mod pack;
mod frame;

#[cfg(test)]
mod tests {
    use crate::socket::{send, SocketPool};
    use crate::pack::unpack;

    #[test]
    fn it_works() {
        let pool = SocketPool::new("127.0.0.1:6381", 1);
        let mut socket = pool.get().unwrap();
        let res = send(&mut socket, "ls '*'\0".as_bytes());

        let r = unpack(res.unwrap().as_slice()).unwrap();
        pool.put(socket);
        println!("{:?}", r);
    }
}
