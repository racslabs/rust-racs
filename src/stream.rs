use std::io::{Cursor, Write};
use rmp::encode::{write_array_len, write_bin};

use crate::command::Command;
use crate::pack::{unpack, Type};
use crate::frame::Frame;
use crate::socket::send;
use crate::utils::{pack, compress};

pub struct Stream<'a> {
    pub(crate) stream_id: String,
    pub(crate) chunk_size: u16,
    pub(crate) batch_size: u32,
    pub(crate) compression: bool,
    pub(crate) compression_level: i32,
    pub(crate) command: &'a Command,
}

impl<'a> Stream<'a> {
    pub fn stream_id(&mut self, stream_id: &str) -> &mut Self {
        self.stream_id = stream_id.to_string();
        self
    }

    pub fn chunk_size(&mut self, chunk_size: u16) -> &mut Self {
        self.chunk_size = chunk_size;
        self
    }

    pub fn batch_size(&mut self, batch_size: u32) -> &mut Self {
        self.batch_size = batch_size;
        self
    }

    pub fn compression(&mut self, compression: bool) -> &mut Self {
        self.compression = compression;
        self
    }

    pub fn compression_level(&mut self, compression_level: i32) -> &mut Self {
        self.compression_level = compression_level;
        self
    }

    pub fn execute(&mut self, data: &[i32]) -> Result<(), String> {
        self.stream_impl(
            &self.stream_id,
            self.chunk_size,
            data,
            self.batch_size,
            self.compression,
            self.compression_level
        )
    }

    fn stream_impl(
        &self,
        stream_id: &str,
        chunk_size: u16,
        data: &[i32],
        batch_size: u32,
        compression: bool,
        compression_level: i32

    ) -> Result<(), String> {
        let bit_depth = match self.command.execute_command(format!("META '{}' 'bit_depth'", stream_id).as_str())? {
            Type::Int(result)     => result as u16,
            Type::Error(result) => return Err(result),
            _                          => return Err("Unknown error.".to_string()),
        };

        self.command.execute_command(format!("OPEN '{}'", stream_id).as_str())?;

        let flags = if compression { 1 } else { 0 };
        let frame = Frame::new(stream_id, flags);

        let n = chunk_size / (bit_depth / 8);
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

            let mut socket = self.command.pool.get().unwrap();
            let response = send(&mut socket, buf.as_slice())?;
            self.command.pool.put(socket);

            let _type = unpack(response.as_slice())?;
            Ok(())
        };

        for chunk in data.chunks(n as usize) {
            let mut block = pack(chunk, bit_depth);

            if compression {
                let compressed = compress(&block.unwrap(), compression_level);
                block = Some(compressed);
            }

            let encoded_frame = frame.pack(&block.unwrap());
            frames.push(encoded_frame);

            if frames.len() == batch_size as usize {
                flush_frames(&mut frames)?;
            }
        }

        flush_frames(&mut frames)?;
        self.command.execute_command(format!("CLOSE '{}'", stream_id).as_str())?;

        Ok(())
    }
}