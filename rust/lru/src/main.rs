use std::collections::HashSet;
use std::vec::Vec;

struct LRUCache {
    num_elts: usize,
    cur_elts: usize,
    look_up: HashSet<u32>,
    lru_list: Vec<u32>,
}
enum InsertType {
    OverWrite,
    New,
}

impl LRUCache {
    fn new(num_elts: usize) -> Self {
        let lru_map = Vec::with_capacity(num_elts);
        Self {
            num_elts,
            cur_elts: 0,
            look_up: HashSet::new(),
            lru_list: lru_map,
        }
    }

    // returns true false to simulate a cache hit/miss
    // we don't have KV semantics - this is not required for the interview
    fn lookup(&mut self, elt: u32) -> bool {
        match self.look_up.contains(&elt) {
            true => {
                let pos = self.lru_list.iter().position(|&x| x == elt).unwrap();
                self.lru_list.remove(pos);
                self.lru_list.push(elt);
                true
            }
            false => false,
        }
    }

    fn evict_(&mut self, count: usize) {
        let mut count = count;
        while self.lru_list.is_empty() == false && count != 0 {
            let t = self.lru_list.remove(0);
            let m = self.look_up.remove(&t);
            assert!(m == true);
            count -= 1;
        }
    }

    fn insert_(&mut self, elt: u32) -> InsertType {
        if self.cur_elts == self.num_elts {
            self.evict_(1);
        }

        // want the head to be least recently used element
        self.lru_list.push(elt);
        self.look_up.insert(elt);
        self.cur_elts += 1;
        InsertType::New
    }

    // insert a element into the cache, elements are supposed to be unique
    fn insert(&mut self, elt: u32) -> InsertType {
        let p = self.look_up.get(&elt);
        match p {
            Some(_) => InsertType::OverWrite,
            None => self.insert_(elt),
        }
    }
}

fn main() {
    let mut pointee = 5;
    let mut r: &mut i32 = &mut pointee;

    let mut pointee2 = 5u32;
    let p: *const u32 = &mut pointee2 as *const u32;

    let mut l = LRUCache::new(3);
    assert!(matches!(l.insert(0), InsertType::New));
    assert!(matches!(l.insert(0), InsertType::OverWrite));
    assert!(matches!(l.insert(1), InsertType::New));
    assert!(matches!(l.insert(2), InsertType::New));
    assert!(matches!(l.insert(3), InsertType::New));
    assert!(l.lookup(0) == false);
    assert!(l.lookup(1) == true);
    assert!(l.lookup(2) == true);
    assert!(l.lookup(3) == true);
}
