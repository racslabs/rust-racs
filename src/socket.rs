use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::usize;


pub struct SocketPool {
    pool: Arc<Mutex<VecDeque<TcpStream>>>,
    size: usize
}

impl SocketPool {

    pub fn new(addr: &str, size: usize) -> Result<Self, String> {
        let pool = Arc::new(Mutex::new(VecDeque::with_capacity(size)));

        for _ in 0..size {
            let addr = SocketAddr::from_str(addr).unwrap();
            let socket = TcpStream::connect(addr).unwrap();

            socket.set_nodelay(true).unwrap();
            pool.lock().unwrap().push_back(socket);
        }

        Ok(Self { pool,  size })
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

    pub fn close(&self) -> Result<(), String> {
        let mut pool = self.pool.lock().unwrap();
        pool.drain(..);
        
        Ok(())
    }
}

pub fn send(stream: &mut TcpStream, data: &[u8]) -> Result<Vec<u8>, String> {
    let prefix = data.len().to_le_bytes();

    {
        let mut writer = BufWriter::new(&mut *stream);
        writer.write_all(&prefix).unwrap();
        writer.write_all(data).unwrap();
        writer.flush().unwrap();
    }

    let mut reader = BufReader::new(stream);

    let mut len_buf = [0u8; 8];
    reader.read_exact(&mut len_buf).unwrap();
    let len = u64::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).unwrap();

    Ok(buf)
}