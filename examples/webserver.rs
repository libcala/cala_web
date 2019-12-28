use std::future::Future;
use cala_web::{WebServer, Stream};

fn request(stream: Stream) -> Box<dyn Future<Output = ()> + Send> {
    Box::new(async {
        let mut stream = stream;

        stream.push_str("This page is not from a file.");
        stream.send().await.unwrap();
    })
}

fn main() {
    WebServer::with_resources("examples/serve")
        .url("/gen", request)
        .start()
}
