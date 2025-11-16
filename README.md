# rust-racs

rust-racs is the rust client library for RACS (Remote Audio Caching Server). 
It provides access to all the RACS commands through a low-level API.


## Basic Operations

### Streaming

```rust
use racs::client::Client;

fn main() {
    // 1️⃣ Connect to a RACS server
    let client = Client::open("127.0.0.1:6381");

    {
        // 2️⃣ Create a pipeline to batch multiple RACS commands
        let mut pipeline = client.pipeline();

        // 3️⃣ Create a new audio stream and open it
        pipeline
            .create(
                "Beethoven Piano Sonata No.1", // stream name
                44100,                         // sample rate (Hz)
                2,                             // number of channels
                16,                            // bit depth
            )
            .open("Beethoven Piano Sonata No.1")
            .execute()  // execute all pipelined commands
            .unwrap();

        // 4️⃣ Reset the pipeline for reuse
        pipeline.reset();
    }

    // 5️⃣ Prepare PCM samples (interleaved L/R, 16- or 24-bit integers)
    let samples: Vec<i32> = /* your PCM audio data */;

    // 6️⃣ Stream audio data to the server
    client
        .stream(
            "Beethoven Piano Sonata No.1",
            1024 * 32,          // chunk size (32 KB)
            samples.as_slice(), // PCM samples
        )
        .unwrap();

    // 7️⃣ Close the stream when finished
    client
        .pipeline()
        .close("Beethoven Piano Sonata No.1")
        .execute()
        .unwrap();

    // 8️⃣ Close the client connection
    client.close();
}

```