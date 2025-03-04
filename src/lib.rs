use std::io;

pub fn new<'a>() -> ltrait::source::Source<'a, String> {
    let lines: Vec<_> = io::stdin().lines().filter_map(|c| c.ok()).collect();

    Box::pin(ltrait::tokio_stream::iter(lines))
}
