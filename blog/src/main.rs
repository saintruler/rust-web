use std::net::TcpListener;

use spider::handle_client;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
