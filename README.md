# async-pipe-rs

[![crates.io](https://img.shields.io/crates/v/async-rs.svg)](https://crates.io/crates/async-rs)
[![Documentation](https://docs.rs/async-rs/badge.svg)](https://docs.rs/async-rs)
[![MIT](https://img.shields.io/crates/l/async-rs.svg)](./LICENSE)

Creates an asynchronous piped reader and writer pair using `tokio.rs`.

[Docs](https://docs.rs/async-rs)

## Example

```rust
use async_pipe;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let (mut w, mut r) = async_pipe::pipe();

    tokio::spawn(async move {
        w.write_all(b"hello world").await.unwrap();
    });

    let mut v = Vec::new();
    r.read_to_end(&mut v).await.unwrap();
    println!("Received: {:?}", String::from_utf8(v));
}
```

## Contributing

Your PRs and stars are always welcome.