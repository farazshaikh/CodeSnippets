use std::{borrow::Borrow, sync::Arc};

// Define a trait that the generic type must implement
trait MyTrait {
    fn print(&self);
}

// Implement the trait for i32
impl MyTrait for i32 {
    fn print(&self) {
        println!("Value: {}", self);
    }
}

// Implement the trait for f64
impl MyTrait for f64 {
    fn print(&self) {
        println!("Value: {}", self);
    }
}

// Define a generic struct with fields wrapped in Arc
struct Data<'a, T: MyTrait + Clone> {
    value: &'a T,
}

// Implement methods for the generic struct
impl<'a, T: MyTrait + Clone> Data<'a, T> {
    fn new(value: &'a T) -> Self {
        Data { value }
    }

    fn get_value(&self) -> Arc<T> {
        self.value.clone()
    }
}

// Build function that returns either i32 or f64 based on a bool input
fn build(use_i32: bool) -> Box<dyn MyTrait> {
    if use_i32 {
        Box::new(42) as Box<dyn MyTrait>
    } else {
        Box::new(3.14) as Box<dyn MyTrait>
    }
}

fn main() {
    // Create a generic Data object wrapped in an Arc using build function
    let use_i32 = true;
    let my_trait: Box<dyn MyTrait> = build(use_i32);
    let data = Data::new(my_trait.borrow() as &dyn MyTrait);

    // Access the value in the main thread
    data.get_value().print();
}
