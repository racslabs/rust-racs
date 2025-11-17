use crate::command::Command;
use crate::pack::Types;
use crate::pipeline::Pipeline;
use crate::socket::SocketPool;


const DEFAULT_POOL_SIZE: usize = 3;

pub struct Client {
    command: Command
}

impl Client {
    pub fn open(address: &str) -> Result<Self, String> {
        let pool = SocketPool::new(address, DEFAULT_POOL_SIZE)?;
        Ok(
            Self {
                command: Command::new(pool)
            }
        )

    }

    pub fn open_with_pool_size(address: &str, pool_size: usize) -> Result<Self, String> {
        let pool = SocketPool::new(address, pool_size)?;
        Ok(
            Self {
                command: Command::new(pool)
            }
        )
    }

    pub fn execute_command(&self, command: &str) -> Result<Types, String> {
        self.command.execute_command(command)
    }

    pub fn stream(&mut self, stream_id: &str, chunk_size: u16, data: &[i32]) ->  Result<(), String> {
        self.command.stream(stream_id, chunk_size, data)
    }

    pub fn pipeline(&'_ self) -> Pipeline<'_> {
        Pipeline::new(&self.command)
    }

    pub fn close(self) -> Result<(), String> {
        self.command.get_pool().close()
    }
}

