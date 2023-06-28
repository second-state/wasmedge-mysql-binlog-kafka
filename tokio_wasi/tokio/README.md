# Tokio for WebAssembly

A runtime for writing reliable, asynchronous, and slim applications with
the Rust programming language. 
This is a fork of the [original tokio](https://github.com/tokio-rs/tokio) so that it can be compiled into
WebAssembly. The WebAssembly app can run inside the [WasmEdge Runtime](https://github.com/WasmEdge/WasmEdge#readme)
as a lightweight and secure alternative to natively compiled apps in Linux container.

* **Fast**: Tokio's zero-cost abstractions give you bare-metal
  performance.

* **Reliable**: Tokio leverages Rust's ownership, type system, and
  concurrency model to reduce bugs and ensure thread safety.

* **Scalable**: Tokio has a minimal footprint, and handles backpressure
  and cancellation naturally.

## Overview

Tokio is an event-driven, non-blocking I/O platform for writing
asynchronous applications with the Rust programming language. At a high
level, it provides a few major components:

* A multithreaded, work-stealing based task [scheduler].
* A reactor backed by the operating system's event queue (epoll, kqueue,
  IOCP, etc...).
* Asynchronous [TCP and UDP][net] sockets.

These components provide the runtime components necessary for building
an asynchronous application.

[net]: https://docs.rs/tokio/latest/tokio/net/index.html
[scheduler]: https://docs.rs/tokio/latest/tokio/runtime/index.html

## Example

A basic TCP echo server with Tokio.

Make sure you activated the full features of the tokio crate on Cargo.toml:

```toml
[dependencies]
tokio_wasi = { version = "1.25", features = ["full"] }
```
Then, on your main.rs:

```rust,no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

More examples can be found [here][examples]. For a larger "real world" example, see the
[mini-redis] repository.

[examples]: https://github.com/tokio-rs/tokio/tree/master/examples
[mini-redis]: https://github.com/tokio-rs/mini-redis/

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you, shall be licensed as MIT, without any additional
terms or conditions.
