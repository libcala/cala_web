use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

async fn request(mut stream: Arc<cala_web::Stream>) {
    let stream = Arc::get_mut(&mut stream).unwrap();

    stream.push_str("This page is not from a file.");
    stream.send().await.unwrap();
}

fn request2(stream: Arc<cala_web::Stream>) -> Box<dyn Future<Output = ()> + Send + 'static> {
    Box::new(request(stream))
}

fn main() {
    let mut map = HashMap::<&str, (&str, cala_web::ResourceGenerator)>::new();
    map.insert("/gen", ("text/html; charset=utf-8", request2));

    cala_web::start("examples/serve", map);
}
