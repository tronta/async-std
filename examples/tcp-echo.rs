//! TCP echo server.
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc localhost 8080
//! ```

#![feature(async_await)]

use async_std::{io, net, prelude::*, task};

async fn process(stream: net::TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);

    let (reader, writer) = &mut (&stream, &stream);
    io::copy(reader, writer).await?;

    Ok(())
}

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = net::TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                process(stream).await.unwrap();
            });
        }
        Ok(())
    })
}
