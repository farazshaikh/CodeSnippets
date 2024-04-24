use std::path::PathBuf;

macro_rules! build_path {
    ($p:ident, $prefix:literal, $idx:expr) => {
        $p.join(format!($prefix, $idx))
    };
}

fn main() {
    let path = PathBuf::from("/tmp");
    let s10 = build_path!(path, "./state/state_{:X}", 10);
    println!("Hello, world! {s10:?}");
}
