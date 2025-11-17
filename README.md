# rust-racs

rust-racs is the rust client library for RACS (Remote Audio Caching Server). 
It provides access to all the RACS commands through a low-level API.


## Basic Operations

To open a connection, simply create a client using ``open``. 

```rust
use racs::Client;

let client = Client::open("127.0.0.1:6381");
```

The ``open`` function creates a client with a default connection pool size of 3. 
To specify the connection pool size, use ``open_with_pool_size``.
```rust
use racs::Client;

let client = Client::open_with_pool_size("127.0.0.1:6381", 5);
```

### Streaming

The ``pipeline`` function is used to chain together multiple RACS commands and execute them sequentially.
In the below example, a new audio stream is created and opened. Then pcm data is chunked into frames 
and streamed to the RACS server.

```rust
use racs::Client;

// 1️⃣ Connect to a RACS server
let client = Client::open("127.0.0.1:6381");

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
let client = Client::open("127.0.0.1:6381");

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
