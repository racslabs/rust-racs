
use crate::socket::{send, SocketPool};
use crate::pack::{unpack, Type};
pub(crate) use crate::stream::Stream;

pub struct Command {
    pub(crate) pool: SocketPool
}

const DEFAULT_CHUNK_SIZE: u16 = 1024 * 32;
const DEFAULT_BATCH_SIZE: u32 = 50;
const DEFAULT_COMPRESSION_LEVEL: i32 = 3;


impl Command {
    pub(crate) fn new(pool: SocketPool) -> Self {
        Self { pool }
    }

    pub fn stream(&'_ self, stream_id: impl Into<String>) -> Stream<'_> {
        Stream {
            stream_id: stream_id.into(),
            chunk_size: DEFAULT_CHUNK_SIZE,
            batch_size: DEFAULT_BATCH_SIZE,
            compression: false,
            compression_level: DEFAULT_COMPRESSION_LEVEL,
            command: self,
        }
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