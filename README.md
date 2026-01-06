# rust-racs

[![crates.io](https://img.shields.io/crates/v/racs.svg)](https://crates.io/crates/racs)

**rust-racs** is the rust client library for [RACS](https://github.com/racslabs/racs).

## Installation

Run the following Cargo command in your project directory:
```
cargo add racs
```
Or add the following line to your Cargo.toml:
```
racs = "0.1.1"
```

## Basic Operations

To open a connection, simply create a client using ``open``. 

```rust
use racs::Client;

let client = Client::open("127.0.0.1:6381").unwrap();
```

The ``open`` function creates a client with a default connection pool size of 3. 
To specify the connection pool size, use ``open_with_pool_size``.
```rust
use racs::Client;

let client = Client::open_with_pool_size("127.0.0.1:6381", 5).unrwap();
```

### Streaming Audio

The ``pipeline`` function is used to chain together multiple RACS commands and execute them sequentially.
In the below example, a new audio stream is created and opened. Then PCM data is chunked into frames
and streamed to the RACS server.

```rust
use racs::Client;

// Connect to the RACS server
let mut client = Client::open("127.0.0.1:6381").unwrap();

// Create a new audio stream using pipeline
client.pipeline()
    .create("vocals", 44100, 2, 16) // stream-id, sample-rate, channels, bit-depth
    .execute()
    .unwrap();

// Prepare PCM samples (interleaved L/R, 16- or 24-bit integers)
let samples: Vec<i32> = /* your PCM audio data */

// Stream audio data to the server
client.stream("vocals")
    .chunk_size(1024 * 32) // 32 KB
    .batch_size(100)
    .compression(true)
    .compression_level(8)
    .execute(&samples)
    .unwrap();
```

If `chunk_size`, `batch_size`, `compression` and `compression_level` are not provided, the default values will be used.
```rust
// Stream audio data to the server
client.stream("vocals")
    .execute(&samples)
    .unwrap();
```


Stream ids stored in RACS can be queried using the ``list`` command. ``list`` takes a glob pattern and returns a list of streams ids matching the pattern.

```rust
use racs::Client;

let client = Client::open("127.0.0.1:6381").unwrap();

let result = client.pipeline()
    .list("*")
    .execute()
    .unwrap();

// List([String("vocals")])
println!("{:?}", result);
```

### Extracting Audio
The below example extracts a 30-second PCM audio segment using the ``range`` command. It then encodes the data to MP3 and writes the resulting bytes to a file.

```rust
use racs::Client;
use racs::Type;
use std::fs::File;
use std::io::Write;

// Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// Extract PCM data
// Encode to MP3
let result = client.pipeline()
    .range("vocals", 0.0, 30.0) // stream-id, start-time (seconds), duration (seconds)
    .encode("audio/mp3") // mime-type
    .execute()
    .unwrap();

// Write to a file
if let Type::U8V(data) = result {
    let mut file = File::create("vocals.mp3").unwrap();
    file.write_all(&data).unwrap();
}

```

Stream metadata can be queried using the ``meta`` command. ``info`` takes the stream id and metadata attribute as parameters.

```rust
use racs::Client;
use racs::Type;

// Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// Get sample rate attribute for stream
let result = client
    .pipeline()
    .meta("vocals", "sample_rate")
    .execute();

// Print the sample rate
if let Type::Int(sample_rate) = result.unwrap() {
    // 44100
    println!("{:?}", sample_rate);
}
```

``i64`` is returned for all metadata attributes. The supported attributes are:

| Attribute       | Description                                 |
|-----------------|---------------------------------------------|
| `channels`      | Channel count of the audio stream.          |
| `sample_rate`   | Sample rate of the audio stream (Hz).       |
| `bit_depth`     | Bit depth of the audio stream.              |
| `ref`           | Reference timestamp (milliseconds UTC).     |
| `size`          | Size of uncompressed audio stream in bytes. |

### Raw Command Execution

To execute raw command strings, use the ``execute_command`` function.

```rust
use racs::Client;
use racs::Type;

let client = Client::open("127.0.0.1:6381").unwrap();

let result = client.execute_command("EVAL '(+ 1 2 3)'");

if let Type::Int(num) = result.unwrap() {
}
```

Refer to the documentation in [RACS](https://github.com/racslabs/racs) for the commands.


## Type Conversions

Below is a table of conversions for the ``Type`` enum between RACS and rust:

| RACS           | Rust             |
|----------------|------------------|
| `Type::Int`    | `i64`            |
| `Type::Float`  | `f64`            |
| `Type::Bool`   | `bool`           |
| `Type::String` | `String`         |
| `Type::Error`  | `Err`            | 
| `Type::Null`   | N/A              |
| `Type::U8V`    | `Vec<u8>`        |
| `Type::U16V`   | `Vec<u16>`       |
| `Type::S16V`   | `Vec<i16>`       |  
| `Type::U32V`   | `Vec<u32>`       |
| `Type::S32V`   | `Vec<i32>`       |
| `Type::C64V`   | `Vec<Complex32>` |
| `Type::List`   | `Vec<Type>`      |




