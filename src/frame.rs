use uuid::Uuid;

const DEFAULT_FLAGS: u8 = 0x0;

struct Info {
    stream_id: String,
    channels: u16,
    sample_rate: u32,
    bit_depth: u16,
    chunk_size: u16,
}

struct Frame {
    chunk_id: &'static [u8; 3],
    session_id: [u8; 16],
    hash: u64,
    checksum: u32,
    channels: u16,
    sample_rate: u32,
    bit_depth: u16,
    block_size: u16,
    flags: u8,
    block: Vec<u8>
}

impl Frame {
    pub fn new(info: Info, block: &[u8]) -> Self {
        Frame {
            chunk_id: b"rsp",
            session_id: Self::session_id(),
            hash: Self::hash(info.stream_id.as_bytes()),
            checksum: crc32c::crc32c(block),
            channels: info.channels,
            sample_rate: info.sample_rate,
            bit_depth: info.bit_depth,
            block_size: info.chunk_size,
            flags: DEFAULT_FLAGS,
            block: block.try_into().unwrap(),
        }
    }

    fn session_id() -> [u8; 16] {
        *Uuid::new_v4().as_bytes()
    }

    fn hash(bytes: &[u8]) -> u64 {
        let (h1, _) = mur3::murmurhash3_x64_128(bytes, 0);
        h1
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.chunk_id.as_ref());
        bytes.extend_from_slice(self.session_id.as_ref());
        bytes.extend_from_slice(self.hash.to_le_bytes().as_ref());
        bytes.extend_from_slice(self.checksum.to_le_bytes().as_ref());
        bytes.extend_from_slice(self.channels.to_le_bytes().as_ref());
        bytes.extend_from_slice(self.sample_rate.to_le_bytes().as_ref());
        bytes.extend_from_slice(self.bit_depth.to_le_bytes().as_ref());
        bytes.extend_from_slice(self.block_size.to_le_bytes().as_ref());
        bytes.push(self.flags);
        bytes.extend_from_slice(self.block.as_slice());
        bytes
    }
}