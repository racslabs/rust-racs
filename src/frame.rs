
pub use crate::command::Info;
use crate::utils;

const DEFAULT_FLAGS: u8 = 0x0;


pub(crate) struct Frame {
    chunk_id: &'static [u8; 3],
    session_id: [u8; 16],
    hash: u64,
    channels: u16,
    sample_rate: u32,
    bit_depth: u16,
    flags: u8,
}

impl Frame {
    pub fn new(info: &Info) -> Self {
        Frame {
            chunk_id: b"rsp",
            session_id: utils::session_id(),
            hash: utils::hash(info.stream_id.as_bytes()),
            channels: info.channels,
            sample_rate: info.sample_rate,
            bit_depth: info.bit_depth,
            flags: DEFAULT_FLAGS,
        }
    }

    pub fn pack(&self, block: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.chunk_id.as_ref());
        bytes.extend_from_slice(self.session_id.as_ref());
        bytes.extend_from_slice(self.hash.to_le_bytes().as_ref());
        bytes.extend_from_slice(crc32c::crc32c(block).to_le_bytes().as_ref());
        bytes.extend_from_slice(self.channels.to_le_bytes().as_ref());
        bytes.extend_from_slice(self.sample_rate.to_le_bytes().as_ref());
        bytes.extend_from_slice(self.bit_depth.to_le_bytes().as_ref());
        bytes.extend_from_slice((block.len() as u16).to_le_bytes().as_ref());
        bytes.push(self.flags);
        bytes.extend_from_slice(block);
        bytes
    }
}