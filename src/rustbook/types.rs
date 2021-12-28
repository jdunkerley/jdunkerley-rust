pub fn run() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let b = &a[1..3];
    println!("{:?}", b);

    let (fa, fb, fc) = (0.1_f32, 0.2_f32, 0.3_f32);
    println!("{:} + {:} = {:x}", fa, fb, (fa + fb).to_bits());
    println!("{:} = {:x}", fc, fc.to_bits());

    let (fa, fb, fc): (f64, f64, f64) = (0.1, 0.2, 0.3); // f64 by default
    println!("{:} + {:} = {:x}", fa, fb, (fa + fb).to_bits());
    println!("{:} = {:x}", fc, fc.to_bits());

    assert!((fa + fb - fc).abs() < f64::EPSILON);
}