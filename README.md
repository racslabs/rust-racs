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

// 1️⃣ Connect to a RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// 2️⃣ Create a new audio stream and open it using pipeline
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

// 3️⃣ Prepare PCM samples (interleaved L/R, 16- or 24-bit integers)
let samples: Vec<i32> = /* your PCM audio data */;

// 4️⃣ Stream audio data to the server
client
    .stream(
        "Beethoven Piano Sonata No.1",
        1024 * 32,          // chunk size (32 KB)
        samples.as_slice(), // PCM samples
    )
    .unwrap();

// 5️⃣ Close the stream when finished
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
use chrono::{DateTime, Days, TimeZone, Utc};
use racs::Client;
use racs::Types;

// 1️⃣ Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// 2️⃣ Get the reference timestamp (in milliseconds)
let result = client
    .pipeline()
    .info("Beethoven Piano Sonata No.1", "ref")
    .execute();

if let Types::Int(ref_ms) = result.unwrap() {

    // 3️⃣ Convert milliseconds to DateTime<Utc>
    let from = Utc.timestamp_millis_opt(ref_ms).unwrap();

    // 4️⃣ Compute end time by adding one day 
    let to = from
        .checked_add_days(Days::new(1))
        .unwrap();

    // 5️⃣ Extract PCM data between `from` and `to`
    // 6️⃣ Convert (format) the audio to MP3
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

    if let Types::U8V(data) = result.unwrap() {

        // 7️⃣ Use or save the MP3 bytes
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

if let Types::S32V(data) = result.unwrap() {
    // Use PCM samples stored in data: Vec<i32>
}
```

### Querying Streams and Metadata

Stream ids stored in RACS can be queried using the ``list`` function.
``list`` takes a glob pattern and returns a vector of streams ids matching the pattern.

```rust
use racs::Client;
use racs::Types;

// 1️⃣ Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// 2️⃣ Run list command matching "*" pattern
let result = client
    .pipeline()
    .list("*")
    .execute();

// 3️⃣ Print the list of stream ids
if let Types::List(list) = result.unwrap() {
    // [Str("Beethoven Piano Sonata No.1")]
    println!("{:?}", list);
}
```

> [!NOTE]
> String is the only element type currently supported for ``Types::List``

Stream metadata can be queried using the ``info`` function. 
``info`` takes the stream id and metadata attribute as parameters.

```rust
use racs::Client;
use racs::Types;

// 1️⃣ Connect to the RACS server
let client = Client::open("127.0.0.1:6381").unwrap();

// 2️⃣ Get sample rate attribute for stream
let result = client
    .pipeline()
    .info("Beethoven Piano Sonata No.1", "sample_rate")
    .execute();

// 3️⃣ Print the sample rate
if let Types::Int(sample_rate) = result.unwrap() {
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
use racs::Types;

let client = Client::open("127.0.0.1:6381").unwrap();

let result = client.execute_command("EXTRACT 'Chopin Etude No.4' 2024-12-20T22:30:45.123Z 2024-12-21T02:56:16.123Z");

if let Types::S32V(data) = result.unwrap() {
    // Use PCM samples stored in data: Vec<i32>
}
```

Refer to the documentation in [RACS](https://github.com/racslabs/racs) for the commands.


## Type Conversions

Below is a table of conversions for the ``Types`` enum between RACS and rust:

| RACS            | Rust       |
|-----------------|------------|
| `Types::Int`    | `i64`      |
| `Types::Float`  | `f64`      |
| `Types::Bool`   | `bool`     |
| `Types::String` | `String`   |
| `Types::Error`  | `Err`      | 
| `Types::Null`   | N/A        |
| `Types::U8V`    | `Vec<u8>`  |
| `types::U16V`   | `Vec<u16>` |




