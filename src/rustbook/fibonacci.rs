fn fibonacci(n: u16) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn run() {
    for n in 1..25 {
        println!("Fibonacci {} - {}", n, fibonacci(n));
    }
}
