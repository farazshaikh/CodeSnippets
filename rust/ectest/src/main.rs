use std::{os::unix::thread, process::AnonPipe};

fn main() {
    let p = Arc::new(thread);
    (0..1).into_iter().map(|x| x);
    println!("Hello, world!");
}
