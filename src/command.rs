use crate::socket::{send, SocketPool};
use crate::utils::pack;
use crate::frame::Frame;
use crate::pack::{unpack, Type};


pub struct Command {
    pool: SocketPool
}

impl Command {
    pub(crate) fn new(pool: SocketPool) -> Self {
        Self { pool }
    }

    pub fn stream(&self, stream_id: &str, chunk_size: u16, data: &[i32]) -> Result<(), String> {
        let bit_depth: u16;

        let result = self.execute_command(format!("INFO '{}' 'bit_depth'", stream_id).as_str())?;
        match result {
            Type::Int(v)  => { bit_depth = v as u16 },
            _                   => { return Err("Invalid bit depth".to_string()) }
        }

        let frame = Frame::new(stream_id);
        let n = chunk_size / bit_depth / 8;

        data.chunks(n as usize).for_each(|chunk| {
            let block = pack(chunk, bit_depth);
            let encoded_frame = frame.pack(block.unwrap().as_mut_slice());

            let mut socket = self.pool.get().unwrap();
            let response = send(&mut socket, encoded_frame.as_slice()).unwrap();
            self.pool.put(socket);

            let _type = unpack(response.as_slice()).unwrap();
        });

        Ok(())
    }

    pub fn execute_command(&self, cmd: &str) -> Result<Type, String> {
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