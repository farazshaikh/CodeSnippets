use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
struct TreeNode {
    data: u32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    fn new_(data: u32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { data, left, right })))
    }

    fn new(data: u32) -> TreeLink {
        TreeNode::new_(data, None, None)
    }

    fn set_left(&mut self, data: u32) -> TreeLink {
        self.left = TreeNode::new(data);
        self.left.clone()
    }

    fn set_right(&mut self, data: u32) -> TreeLink {
        self.right = TreeNode::new(data);
        self.right.clone()
    }

    fn traverse(root: TreeLink) -> Result<(), ()> {
        let root = root.ok_or(())?;
        println!("{}", root.borrow().data);
        let _ = TreeNode::traverse(root.borrow().left.clone());
        let _ = TreeNode::traverse(root.borrow().right.clone());
        Ok(())
    }

    fn bfs(root: TreeLink) -> Result<(), ()> {
        let root = root.ok_or(())?;
        let mut q = VecDeque::new();
        q.push_back((root.clone(), 0));
        let mut clvl = 0;
        print!("lvl {clvl}");
        while !q.is_empty() {
            let (p, lvl) = q.pop_front().unwrap();
            let p = p.borrow();
            if clvl != lvl {
                println!();
                print!("lvl {lvl}");
                clvl = lvl;
            }
            print!(" {} ", p.data);
            if let Some(l) = &p.left {
                q.push_back((l.clone(), lvl + 1))
            }
            if let Some(r) = &p.right {
                q.push_back((r.clone(), lvl + 1))
            }
        }
        Ok(())
    }
}

fn main() {
    /*
        0
     1   2
      3 4  5
    */
    let zero = TreeNode::new(0).unwrap();
    let one = zero.borrow_mut().set_left(1).unwrap();
    let two = zero.borrow_mut().set_right(2).unwrap();
    one.borrow_mut().set_right(3);
    two.borrow_mut().set_left(4);
    two.borrow_mut().set_right(5);

    println!("Pre-order");
    let _ = TreeNode::traverse(Some(zero.clone()));
    println!("BFS");
    let _ = TreeNode::bfs(Some(zero));
}
