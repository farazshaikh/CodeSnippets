use std::fs::File;
fn main() -> Result<(), ()> {
    let p: u32 = 1;
    let p = File::open("~/.emacs").unwrap();
    for n in 0..10 {
        let n = n.to_string();
        let n = n.as_str();
        dbg!(n);
    }
    let p: Vec<_> = (0..10).into_iter().map(|x| x.to_string()).collect();
    p.insert(0, "faraz".as_str());
    p.Ok(())
}
