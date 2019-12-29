use cala_web::{WebServer, Stream};

async fn request(stream: Stream) -> Result<(), std::io::Error> {
    stream.push_str("This page is not from a file.");
    stream.send().await
}

fn main() {
    WebServer::with_resources("examples/serve")
        .url("/gen", request)
        .start()
}
