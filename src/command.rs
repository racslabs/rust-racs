use crate::socket::{send, SocketPool};
use crate::utils::pack;
use crate::frame::Frame;
use crate::pack::{unpack, Types};


pub struct Info {
    pub stream_id: String,
    pub channels: u16,
    pub sample_rate: u32,
    pub bit_depth: u16,
    pub chunk_size: u16,
}

pub struct Command {
    pool: SocketPool
}

impl Command {
    pub(crate) fn new(pool: SocketPool) -> Self {
        Self { pool }
    }

    pub fn stream(&self, info: &Info, data: &[i32]) -> Result<(), String> {
        let frame = Frame::new(info);
        let n = info.chunk_size / info.bit_depth / 8;

        data.chunks(n as usize).for_each(|chunk| {
            let block = pack(chunk, info.bit_depth);
            let encoded_frame = frame.pack(block.unwrap().as_mut_slice());

            let mut socket = self.pool.get().unwrap();
            let response = send(&mut socket, encoded_frame.as_slice()).unwrap();
            self.pool.put(socket);

            let _type = unpack(response.as_slice()).unwrap();
        });

        Ok(())
    }

    pub fn execute_command(&self, cmd: &str) -> Result<Types, String> {
        let mut socket = self.pool.get().unwrap();
        let mut bytes = cmd.as_bytes().to_vec();

        bytes.push('\0' as u8);
        let response = send(&mut socket, bytes.as_slice()).unwrap();
        self.pool.put(socket);

        unpack(response.as_slice())
    }

    pub fn get_pool(self) -> SocketPool {
        self.pool
    }
}