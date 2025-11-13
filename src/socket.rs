use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::io::{BufReader, Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::usize;


pub struct SocketPool {
    pool: Arc<Mutex<VecDeque<TcpStream>>>,
    size: usize
}

impl SocketPool {

    pub fn new(addr: &str, size: usize) -> Self {
        let pool = Arc::new(Mutex::new(VecDeque::with_capacity(size)));

        for _ in 0..size {
            let addr = SocketAddr::from_str(addr).unwrap();
            let socket = TcpStream::connect(addr).unwrap();

            socket.set_nodelay(true).unwrap();
            pool.lock().unwrap().push_back(socket);
        }


        Self { pool,  size}
    }

    pub fn get(&self) -> Option<TcpStream> {
        self.pool.lock().unwrap().pop_front()
    }

    pub fn put(&self, stream: TcpStream) {
        self.pool.lock().unwrap().push_back(stream);
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn close(&self) {
        let mut pool = self.pool.lock().unwrap();
        pool.drain(..);
    }
}

pub fn send(stream: &mut TcpStream, data: &[u8]) -> Result<Vec<u8>> {
    let prefix = data.len().to_le_bytes();
    stream.write_all(&prefix)?;
    stream.write_all(data)?;
    stream.flush()?;

    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf)?;
    let len = u64::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    BufReader::new(stream).read_exact(&mut buf)?;

    Ok(buf)
}