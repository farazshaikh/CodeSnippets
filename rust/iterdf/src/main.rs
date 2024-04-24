#[derive(Debug, Copy, Clone)]
struct Bar {
    k: u32,
}

fn print_typeof<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>())
}

fn iter_foo(i: impl IntoIterator<Item = Bar>) {
    for bar in i.into_iter() {
        println!("{bar:?}");
    }
}

fn main() {
    let m = Box::new([Bar { k: 1 }, Bar { k: 2 }]);
    println!("Hello, world!");

    loop {
        iter_foo(*m);
    }
}
