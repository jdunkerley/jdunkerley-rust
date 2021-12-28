use crate::mandelbrot::complex::Complex;
use num::Zero;

const MAX_ITERS: u16 = 700;
const HEIGHT: usize = 48;
const WIDTH: usize = 120;

fn steps_to_char(steps: u16) -> char {
    match steps {
        0..=2 => ' ',
        3..=5 => '.',
        6..=10 => '•',
        11..=30 => '*',
        31..=100 => '+',
        101..=200 => 'x',
        201..=400 => '$',
        401..=MAX_ITERS => '#',
        _ => '%',
    }
}

fn eval_point(real: f32, imaginary: f32) -> u16 {
    let mut z = Complex::zero();
    let c = Complex { real, imaginary };

    for i in 0..MAX_ITERS {
        if z.absolute() > 2_f32 {
            return i;
        }
        z = z * z + c;
    }

    MAX_ITERS
}

pub fn run() {
    for y_step in 0..HEIGHT {
        for x_step in 0..WIDTH {
            let steps = eval_point(
                -2_f32 + 3_f32 * (x_step as f32 / WIDTH as f32),
                -1_f32 + 2_f32 * (y_step as f32 / HEIGHT as f32),
            );
            let char = steps_to_char(steps);
            print!("{}", char);
        }
        println!();
    }
}