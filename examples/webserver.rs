use std::collections::HashMap;

async fn request(stream: &mut cala_web::Stream) {
    stream.push_str("This page is not from a file.");
    stream.send().await;
}

fn main() {
    let mut map = HashMap::<&str, cala_web::ResourceGenerator>::new();
    map.insert("/gen", ("text/html; charset=utf-8", request));

    cala_web::start("examples/serve", map);
}
