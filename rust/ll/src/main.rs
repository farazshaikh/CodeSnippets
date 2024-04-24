
mod cll {
#[derive(Default)]
pub struct Link {
    next: Option<Box<Elt>>
}

struct Elt {
    data: u32,
    next: Link
}

pub fn new_head() -> Link {
    Default::default()
}

pub fn insert(head: &mut Link, data: u32) {
    let next = head.next.take();
    head.next =  Some(Box::new(Elt { data, next: Link{next} }));

}

pub fn trav(head: &Link) {
    let mut trav = &head.next;
    while let Some(elt) = trav{
        println!("Elt {}", elt.data);
        trav = &elt.next.next;
    }
}
}

mod fll {
    pub enum Node {
        Cons(u32, Box<Node>),
        None
    }

    pub fn new() -> Node {
        Node::None
    }

    pub fn push(head: Node, data: u32) -> Node {
        Node::Cons(data, Box::new(head))
    }

    pub fn dump(head: &Node) {
        match head {
            Node::Cons(data, ref next) => { print!("FLL elt {data} "); dump(next); }
            Node::None => {}
        }
    }
}

mod flt {
    enum Node {
        Cons(u32, Option<Box<Node>>, Option<Box<Node>>),
        Nil,
    }

    fn new() -> Box<Node> {
        Box::new(Node::Nil)
    }

    fn new_node(data:u32) -> Box<Node> {
        Box::new(Node::Cons(data, None, None))
    }

    fn insert(data: u32, mut node:   Box<Node>) -> Box<Node> {
        match &mut *node {
            Node::Nil => { new_node(data) },
            Node::Cons(n_data,  n_left,  n_right) => {
                    if data >= *n_data {
                        let right = n_right.take();
                        match right {
                         None => {
                             n_right.replace(new_node(data));
                         }
                         Some(node) => {
                            n_right.replace(insert(data, node));
                        }
                    }
                }
                node
            }
        }
    }

}

mod tree {
    enum Node {
        Cons(u32, Option<Node>, Option<Node>),
        Nil,
    }
}

fn main() {
    println!("Hello, world!");
    let mut h = cll::new_head();
    (0..100).for_each(|data| cll::insert(&mut h, data));
    cll::trav(&h);
    let mut head = fll::new();
    for x in  0..100 {
        head = fll::push(head, x);
    }
    fll::dump(&head);
}
