# rust-racs

**rust-racs** is the rust client library for [RACS](https://github.com/racslabs/racs). 
It provides access to all the RACS commands through a low-level API.


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

### Streaming

The ``pipeline`` function is used to chain together multiple RACS commands and execute them sequentially.
In the below example, a new audio stream is created and opened. Then PCM data is chunked into frames 
and streamed to the RACS server.

```rust
use racs::Client;

// Connect to a RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// Create a new audio stream and open it using pipeline
client
    .pipeline()
    .create(
        "Beethoven Piano Sonata No.1", // stream name
        44100,                         // sample rate (Hz)
        2,                             // number of channels
        16,                            // bit depth
    )
    .open("Beethoven Piano Sonata No.1")
    .execute()  // execute all pipelined commands
    .unwrap();

// Prepare PCM samples (interleaved L/R, 16- or 24-bit integers)
let samples: Vec<i32> = /* your PCM audio data */;

// Stream audio data to the server
client
    .stream(
        "Beethoven Piano Sonata No.1",
        1024 * 32,          // chunk size (32 KB)
        samples.as_slice(), // PCM samples
    )
    .unwrap();

// Close the stream when finished
client
    .pipeline()
    .close("Beethoven Piano Sonata No.1")
    .execute()
    .unwrap();
```

### Extracting and Formating
The below example retrieves a reference timestamp, and uses it to extract an audio segment based on the given range. 
It then converts the extracted PCM data into MP3 format and writes the resulting bytes to a file.

```rust
use chrono::{Days, TimeZone, Utc};
use racs::Client;
use racs::Type;
use std::fs::File;
use std::io::Write;

// Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// Get the reference timestamp (in milliseconds)
let result = client
    .pipeline()
    .info("Beethoven Piano Sonata No.1", "ref")
    .execute();

if let Type::Int(ref_ms) = result.unwrap() {

    // Convert milliseconds to DateTime<Utc>
    let from = Utc.timestamp_millis_opt(ref_ms).unwrap();

    // Compute end time by adding one day 
    let to = from
        .checked_add_days(Days::new(1))
        .unwrap();

    // Extract PCM data between `from` and `to`
    // Convert (format) the audio to MP3
    let result = client
        .pipeline()
        .extract("Beethoven Piano Sonata No.1", from, to)
        .format(
            "audio/mp3", // mime-type
            44100,       // sample rate (Hz)
            2,           // number of channels
            16           // bit depth
        )
        .execute();

    if let Type::U8V(data) = result.unwrap() {

        // Use or save the MP3 bytes
        // e.g. write them to a file
        let mut file = File::create("beethoven.mp3").unwrap();
        file.write_all(&data).unwrap();
    }
}

```

To extract PCM data without formating, do the following instead:

```rust
// Extract PCM data between `from` and `to`
let result = client
    .pipeline()
    .extract("Beethoven Piano Sonata No.1", from, to)
    .execute();

if let Type::S32V(data) = result.unwrap() {
    // Use PCM samples stored in data: Vec<i32>
}
```

### Querying Streams and Metadata

Stream ids stored in RACS can be queried using the ``list`` function.
``list`` takes a glob pattern and returns a vector of streams ids matching the pattern.

```rust
use racs::Client;
use racs::Type;

// Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// Run list command matching "*" pattern
let result = client
    .pipeline()
    .list("*")
    .execute();

// Print the list of stream ids
if let Type::List(list) = result.unwrap() {
    // [Str("Beethoven Piano Sonata No.1")]
    println!("{:?}", list);
}
```

> [!NOTE]
> ``String`` is the only element type currently supported for ``Types::List``

Stream metadata can be queried using the ``info`` function. 
``info`` takes the stream id and metadata attribute as parameters.

```rust
use racs::Client;
use racs::Type;

// Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// Get sample rate attribute for stream
let result = client
    .pipeline()
    .info("Beethoven Piano Sonata No.1", "sample_rate")
    .execute();

// Print the sample rate
if let Type::Int(sample_rate) = result.unwrap() {
    println!("{:?}", sample_rate);
}
```

``i64`` is returned for all metadata attributes. The supported attributes are:

| Attribute       | Description                                |
|-----------------|--------------------------------------------|
| `channels`      | Channel count of the audio stream.         |
| `sample_rate`   | Sample rate of the audio stream (Hz).      |
| `bit_depth`     | Bit depth of the audio stream.             |
| `ref`           | Reference timestamp (milliseconds UTC).    |
| `size`          | Size of audio stream in bytes.             |

### Raw Command Execution

To execute raw command strings, use the ``execute_command`` function.

```rust
use racs::Client;
use racs::Type;

let client = Client::open("127.0.0.1:6381").unwrap();

let result = client.execute_command("EVAL '(+ 1 2 3)'");

if let Type::INT(num) = result.unwrap() {
}
```

Refer to the documentation in [RACS](https://github.com/racslabs/racs) for the commands.


## Type Conversions

Below is a table of conversions for the ``Types`` enum between RACS and rust:

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
| `Type::List`   | `Vec<Types>`     |




