// Iterator that works but has a use after drop bug. This should be flagged by MIRI
fn tableuseafterfree() {
    let journal = redbfoo::tableleakbug::Journal::open("/tmp/jnltableleak.db").unwrap();
    journal.insertdata().unwrap();
    let iter = journal.iter(0, 3).unwrap();
    for item in iter {
        println!("{item:?}");
    }
}

// Iterator on the Journal based on
// blog: https://morestina.net/blog/1868/self-referential-types-for-fun-and-profit
fn morestina() {
    let journal = redbfoo::morestina::Journal::open("/tmp/jnl.db").unwrap();
    journal.insertdata().unwrap();
    for item in journal.range_iter(0, 3).unwrap() {
        println!("{item:?}");
    }
}

fn main() {
    tableuseafterfree();
    morestina();
}
