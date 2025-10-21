use std::collections::VecDeque;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, Result};

pub struct ConnectionPool {
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

pub fn send(stream: &mut TcpStream, data: &[u8]) -> Result<Vec<u8>> {
    let prefix = (data.len() as u64).to_le_bytes();

    let mut message = Vec::with_capacity(8 + data.len());
    message.extend_from_slice(&prefix);
    message.extend_from_slice(data);

    stream.write_all(&message)?;
    stream.flush()?;

    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf)?;
    let len = u64::from_le_bytes(buf);

    let mut buf = vec![0u8; len as usize];
    stream.read_exact(&mut buf)?;

    Ok(buf)
}
