use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::LinkedList;
fn main() {
    let s = 10;
    println!("Hello, world!");
    let p = 100;
    let mut m = (0..100).map(|i| (i, i)).collect::<BTreeMap<_, _>>();
}

trait Bar {
    fn a(self);
    fn aq(self);
    fn aqs(self);
}

struct nn();

impl Bar for nn {
    fn a(self) {
        todo!()
    }

    fn aq(self) {
        todo!()
    }

    fn aqs(self) {
        todo!()
    }
}

#[test]
fn test1() {
    let mut t = (0..100).collect::<Vec<_>>();
    let idx = t.iter().position(|&x| x == 2);
    match idx {
        Some(idx) => t.remove(idx),
        None => todo!(),
    };
    println!("{:?}", t);
}

#[test]
fn linkedlist() {
    let mut ll = LinkedList::new();
    (0..100).for_each(|x| ll.push_back(x));
    let mm = (0..100).collect::<LinkedList<_>>();
    let mut hm = (0..100).map(|x| (x, x)).collect::<HashMap<_, _>>();
    for x in &hm {
        println!("{x:?}");
    }

    hm.entry(500).or_insert(500);
    println!("{:?}", mm);
}
