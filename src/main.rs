use num::Num;

fn compute_regression<I, T>(x_vals: I, y_vals: I) -> Option<(T, T)>
where
    I: IntoIterator<Item=T>,
    T: Num + Copy
{
    let mut x_iter = x_vals.into_iter();
    let mut y_iter = y_vals.into_iter();

    let mut x_opt = x_iter.next();
    let mut y_opt = y_iter.next();

    let z = T::zero();
    let z1 = T::one();
    let mut accumulator : (T, T, T, T, T) = (z, z, z, z, z);

    while x_opt.is_some() && y_opt.is_some() {
        let x_val = x_opt.unwrap();
        let y_val = y_opt.unwrap();

        accumulator.0 = accumulator.0 + z1;
        accumulator.1 = accumulator.1 + x_val;
        accumulator.2 = accumulator.2 + (x_val * x_val);
        accumulator.3 = accumulator.3 + y_val;
        accumulator.4 = accumulator.4 + (x_val * y_val);

        x_opt = x_iter.next();
        y_opt = y_iter.next();
    }

    if x_opt.is_some() || y_opt.is_some() {
        return None;
    }

    let slope = (accumulator.4 - accumulator.1 * accumulator.3 / accumulator.0) / (accumulator.2 - accumulator.1 * accumulator.1 / accumulator.0);
    let result = (slope, (accumulator.3 - slope * accumulator.1) / accumulator.0);

    return Some(result);
}

fn main() {
    let a = [1, 2, 3, 4];
    let b = [10, 8, 6, 4];

    let reg = compute_regression(a, b).unwrap();
    println!("{:?}", reg);

    println!("Hello, world!");
}
