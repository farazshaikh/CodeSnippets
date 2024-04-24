struct Tp {
    tp: String,
    data: u32,
}

impl Tp {
    fn new(tp: impl AsRef<str>, data: u32) -> Self {
        Self {
            tp: tp.as_ref().into(),
            data,
        }
    }
}

fn main() {
    let m = Tp::new("Third student", 3);
    println!("m {:p} tp {:p} data {:p}", &m, m.tp.as_str(), &m.data);
    let m = Box::new(m);
    println!("m {:p} tp {:p} data {:p}", m, m.tp.as_str(), &m.data);
    let x = m.as_ref();
    println!("x {:p} tp {:p} data {:p}", x, x.tp.as_str(), &x.data);
    let ps = *x;
    println!("ps {:p} tp {:p} data {:p}", &ps, &ps.tp.as_str(), &ps.data);
    println!("Hello, world!");
}
