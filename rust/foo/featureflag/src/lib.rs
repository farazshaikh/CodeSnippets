pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(feature = "special")]
fn special_function() {
    println!("Special compilation");
}

#[cfg(not(feature = "special"))]
fn special_function() {
    println!("not spcial function");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        super::special_function();
    }
}
