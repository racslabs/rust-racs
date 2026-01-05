use std::io::{Cursor, Write};
use rmp::encode::{write_array_len, write_bin};
use crate::socket::{send, SocketPool};
use crate::utils::{pack, compress};
use crate::frame::Frame;
use crate::pack::{unpack, Type};

pub struct Command {
    pool: SocketPool
}

impl Command {
    pub(crate) fn new(pool: SocketPool) -> Self {
        Self { pool }
    }

    pub fn stream(&self, stream_id: &str, chunk_size: u16, data: &[i32], batch_size: u32, compression: bool) -> Result<(), String> {
        let bit_depth = match self.execute_command(format!("META '{}' 'bit_depth'", stream_id).as_str())? {
            Type::Int(v) => v as u16,
            _ => return Err("Invalid bit depth".to_string()),
        };

        let flags = if compression { 1 } else { 0 };
        let frame = Frame::new(stream_id, flags);

        let n = chunk_size / bit_depth / 8;
        let mut frames: Vec<Vec<u8>> = Vec::new();

        let flush_frames = |frames: &mut Vec<Vec<u8>>| -> Result<(), String> {
            if frames.is_empty() {
                return Ok(());
            }

            let mut buf = Vec::new();
            let mut cursor = Cursor::new(&mut buf);

            cursor.write_all(b"rsp").unwrap();
            write_array_len(&mut cursor, frames.len() as u32).unwrap();

            for f in frames.iter() {
                write_bin(&mut cursor, f).unwrap();
            }

            frames.clear();

            let mut socket = self.pool.get().unwrap();
            let response = send(&mut socket, buf.as_slice())?;
            self.pool.put(socket);

            let _type = unpack(response.as_slice())?;
            Ok(())
        };

        for chunk in data.chunks(n as usize) {
            let mut block = pack(chunk, bit_depth);

            if compression {
                let compressed = compress(&block.unwrap());
                block = Some(compressed);
            }

            let encoded_frame = frame.pack(&block.unwrap());
            frames.push(encoded_frame);

            if frames.len() == batch_size as usize {
                flush_frames(&mut frames)?;
            }
        }

        flush_frames(&mut frames)?;

        Ok(())
    }

    pub fn execute_command(&self, cmd: &str) -> Result<Type, String> {
        let mut socket = self.pool.get().unwrap();
        let mut bytes = cmd.as_bytes().to_vec();

        bytes.push('\0' as u8);
        let response = send(&mut socket, bytes.as_slice())?;
        self.pool.put(socket);

        unpack(response.as_slice())
    }

    pub fn get_pool(self) -> SocketPool {
        self.pool
    }
}