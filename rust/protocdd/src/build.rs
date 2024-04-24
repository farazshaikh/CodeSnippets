use prost_build::*;
fn main() {
    compile_protos(&["src/disperser.proto"], &["src/"]).unwrap();
}
