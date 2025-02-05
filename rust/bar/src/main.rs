use std::borrow::Borrow;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

// collapses overlaps in a collection of half open ranges

pub fn collapse_ranges(x: &mut [(u32, u32)]) -> Vec<(u32, u32)> {
    if x.len() == 0 {
        return x.to_vec();
    }
    x.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut ret = Vec::with_capacity(x.len());
    ret.push(x[0]);

    for e in x.into_iter().skip(1) {
        if e.0 >= ret.last().unwrap().1 {
            ret.push(*e);
            continue;
        }
        let u = ret.last_mut().unwrap();
        u.1 = e.1;
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::collapse_ranges;

    #[test]
    fn name() {
        let mut ranges = [(0, 10), (10, 20), (15, 30)];
        let v = collapse_ranges(&mut ranges);
        dbg!(collapse_ranges(&mut ranges));
        assert!(v.len() == 2);
        assert!(v[0] == (0, 10));
        assert!(v[1] == (10, 30));
    }
}

use std::fs::File;
use std::io::{Read, Write};

// fn main() {
//     loop {
//         let mut f = File::create("/tmp/bar.txt").unwrap();
//         let _ = f.write("faraz".to_string().as_bytes());
//         let _ = f.flush();
//         f.set_len(10);
//     }

//     let i = 5;
//     let f = "fraz";
// }

fn main() {
    let i: u32 = 60;
}
