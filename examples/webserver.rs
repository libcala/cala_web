use cala_web::{WebServer, Stream};
use pasts::prelude::*;

async fn request(stream: Stream) -> Result<(), std::io::Error> {
    stream.push_str("This page is not from a file.");
    stream.send().await
}

fn main() {
    // Build the URL tree.
    let webserver = WebServer::with_resources("examples/serve")
        .url("/gen", request);

    // Start the executor.
    pasts::ThreadInterrupt::block_on(webserver)
}
