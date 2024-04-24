fn print_ranges(start: u32, ne_end: u32, step: u32) {
    println!("printing ranges");
    assert!(start <= ne_end);
    let nelts = ne_end - start;
    println!("nelts{nelts}");
    let nelts = nelts + ((step - (nelts % step)) % step);
    println!("nelts adjusted{nelts}");
    for rend in (start..(start + nelts)).rev().step_by(step as usize) {
        let rstart = rend - (step - 1);
        println!("Ex {rstart} {}", rend.min(ne_end - 1));
    }
}

fn main() {
    println!("Hello, world!");

    for start in 0..4 {
        for end in start..20 {
            print_ranges(start, end, 5);
        }
    }
}
