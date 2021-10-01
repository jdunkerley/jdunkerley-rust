pub fn run(a: u8) {
    println!("Testing some overflows... for {}", a);

    let b = u8::checked_add(a, 128);
    if b.is_none() {
        println!("Overflow!");
    } else {
        println!("No Overflow: {}", b.unwrap());
    }

    let c = u8::wrapping_add(a, 128);
    println!("Wrapping: {}", c);

    let c = u8::overflowing_add(a, 128);
    println!("Overflowing: {} {}", c.0, c.1);
}
