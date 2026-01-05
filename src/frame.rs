
use crate::utils;

pub(crate) struct Frame {
    chunk_id: &'static [u8; 3],
    session_id: [u8; 16],
    hash: u64,
    flags: u8,
}

impl Frame {
    pub fn new(stream_id: &str, flags: u8) -> Self {
        Frame {
            chunk_id: b"rsp",
            session_id: utils::session_id(),
            hash: utils::hash(stream_id.as_bytes()),
            flags,
        }
    }

    pub fn pack(&self, block: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.chunk_id.as_ref());
        bytes.extend_from_slice(self.session_id.as_ref());
        bytes.extend_from_slice(self.hash.to_le_bytes().as_ref());
        bytes.extend_from_slice(crc32c::crc32c(block).to_le_bytes().as_ref());
        bytes.extend_from_slice((block.len() as u16).to_le_bytes().as_ref());
        bytes.push(self.flags);
        bytes.extend_from_slice(block);
        bytes
    }
}