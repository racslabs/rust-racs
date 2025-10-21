use std::collections::VecDeque;
use std::net::{ TcpStream };
use std::sync::{Arc, Mutex};


struct ConnectionPool {
    addr: String,
    pool: Arc<Mutex<VecDeque<TcpStream>>>,
    size: usize
}

impl ConnectionPool {

    pub fn new(addr: &str, size: usize) -> ConnectionPool {
        let pool = Arc::new(Mutex::new(VecDeque::with_capacity(size)));

        for _ in 0..size {
            let conn = TcpStream::connect(addr).unwrap();
            pool.lock().unwrap().push_back(conn);
        }

        Self {
            addr: addr.to_string(),
            pool,
            size,
        }
    }

    pub fn get(&self) -> Option<TcpStream> {
        self.pool.lock().unwrap().pop_front()
    }

    pub fn put(&self, stream: TcpStream) {
        self.pool.lock().unwrap().push_back(stream);
    }

    pub fn close(&self) {
        let mut pool = self.pool.lock().unwrap();
        pool.drain(..);
    }

}