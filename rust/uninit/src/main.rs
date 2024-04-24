#![feature(new_uninit)]
fn main() {
    let s0len: u64 = 20;
    let mut s0 = Box::<[u8]>::new_uninit_slice(s0len.try_into().unwrap());
    unsafe {
        let s0 = s0.assume_init();
    }

    println!("Hello, world!");
}
