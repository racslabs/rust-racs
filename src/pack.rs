use std::io::Read;
use rmp::decode;
use num_complex::Complex32;


#[derive(Debug)]
pub enum Types {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Err(String),
    Nil,
    U8V(Vec<u8>),
    U16V(Vec<u16>),
    S16V(Vec<i16>),
    U32V(Vec<u32>),
    S32V(Vec<i32>),
    C64V(Vec<Complex32>),
    List(Vec<Types>)
}

fn unpack_str(reader: &[u8]) -> Result<Types, String> {
    let (v, _) = decode::read_str_from_slice(reader).unwrap();
    Ok(Types::Str(v.to_string()))
}

fn unpack_err(reader: &[u8]) -> Result<Types, String> {
    let (v, _) = decode::read_str_from_slice(reader).unwrap();
    Err(v.to_string())
}

fn unpack_int(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_int(&mut reader).unwrap();
    Ok(Types::Int(v))
}

fn unpack_float(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_f64(&mut reader).unwrap();
    Ok(Types::Float(v))
}

fn unpack_bool(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_bool(&mut reader).unwrap();
    Ok(Types::Bool(v))
}

fn unpack_nil() -> Result<Types, String> {
    Ok(Types::Nil)
}

fn unpack_u8v(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_bin_len(&mut reader).unwrap();
    let mut data = vec![0u8; v as usize];
    reader.read_exact(&mut data).unwrap();
    Ok(Types::U8V(data))
}

fn unpack_u16v(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_bin_len(&mut reader).unwrap();
    let mut data = vec![0u8; v as usize];

    reader.read_exact(data.as_mut_slice()).unwrap();

    let u16v = data.chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();

    Ok(Types::U16V(u16v))
}

fn unpack_s16v(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_bin_len(&mut reader).unwrap();
    let mut data = vec![0u8; v as usize];

    reader.read_exact(data.as_mut_slice()).unwrap();

    let s16v = data.chunks_exact(2)
        .map(|b| i16::from_le_bytes([b[0], b[1]]))
        .collect();

    Ok(Types::S16V(s16v))
}

fn unpack_u32v(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_bin_len(&mut reader).unwrap();
    let mut data = vec![0u8; v as usize];

    reader.read_exact(data.as_mut_slice()).unwrap();

    let u32v = data.chunks_exact(4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect();

    Ok(Types::U32V(u32v))
}

fn unpack_s32v(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_bin_len(&mut reader).unwrap();
    let mut data = vec![0u8; v as usize];

    reader.read_exact(data.as_mut_slice()).unwrap();

    let i32v = data.chunks_exact(4)
        .map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect();


    Ok(Types::S32V(i32v))
}

fn unpack_c64v(mut reader: &[u8]) -> Result<Types, String> {
    let v = decode::read_bin_len(&mut reader).unwrap();
    let mut data = vec![0u8; v as usize];

    reader.read_exact(data.as_mut_slice()).unwrap();

    let c64v = data.chunks_exact(8)
        .map(|b| {
            let re = f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
            let im = f32::from_le_bytes([b[4], b[5], b[6], b[7]]);

            Complex32::new(re, im)
        }).collect();

    Ok(Types::C64V(c64v))
}

fn unpack_list(mut reader: &[u8], n: u32) -> Result<Types, String> {
    let mut v = Vec::new();

    if n > 1 {
        loop {
            let (s, rem) = decode::read_str_from_slice(reader).unwrap();
            v.push(Types::Str(s.into()));

            if rem.is_empty() { break }
            reader = rem;
        }
    }

    v.reverse();
    Ok(Types::List(v))
}

pub fn unpack(buf: &[u8]) -> Result<Types, String> {
    let mut reader: &[u8] = &buf[..];

    let len = decode::read_array_len(&mut reader).unwrap();
    if len < 1 {
        return Err("Invalid buffer length".to_string())
    }

    let (_type, rem) = decode::read_str_from_slice(reader).unwrap();
    reader = rem;

    match _type {
        "string"      => { unpack_str(&mut reader) }
        "error"       => { unpack_err(&mut reader) }
        "bool"        => { unpack_bool(&mut reader) }
        "int"         => { unpack_int(&mut reader) }
        "float"       => { unpack_float(&mut reader) }
        "null"        => { unpack_nil() }
        "u8v" | "s8v" => { unpack_u8v(&mut reader) }
        "u16v"        => { unpack_u16v(&mut reader) }
        "s16v"        => { unpack_s16v(&mut reader) }
        "u32v"        => { unpack_u32v(&mut reader) }
        "s32v"        => { unpack_s32v(&mut reader) }
        "c64v"        => { unpack_c64v(&mut reader) }
        "list"        => { unpack_list(&mut reader, len) }
        _             => Err(format!("Unknown type: {}", _type)),
    }
}
