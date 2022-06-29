use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/rust-uds.sock";
    let mut stream = UnixStream::connect(socket_path)?;
    stream.write_all(b"Hello")?;
    Ok(())
}
