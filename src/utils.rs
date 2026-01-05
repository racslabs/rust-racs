use uuid::Uuid;

const DEFAULT_COMPRESSION_LEVEL: i32 = 3;


pub fn pack(data: &[i32], bit_depth: u16) -> Option<Vec<u8>> {
    match bit_depth {
        16 => {
            let mut out = Vec::with_capacity(data.len() * 2);
            for &x in data {
                out.extend_from_slice(&(x as i16).to_le_bytes());
            }
            Some(out)
        }
        24 => {
            let mut out = Vec::with_capacity(data.len() * 3);
            for &x in data {
                let bytes = x.to_le_bytes();
                out.extend_from_slice(&bytes[..3]);
            }
            Some(out)
        }
        _ => None,
    }
}

pub fn session_id() -> [u8; 16] {
    *Uuid::new_v4().as_bytes()
}

pub fn hash(bytes: &[u8]) -> u64 {
    let (h1, _) = mur3::murmurhash3_x64_128(bytes, 0);
    h1
}

pub fn compress(data: &[u8]) -> Vec<u8> {
    zstd::bulk::compress(data, DEFAULT_COMPRESSION_LEVEL)
        .expect("zstd compress failed")
}
