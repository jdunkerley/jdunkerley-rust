mod misc;
mod rustbook;
mod mandelbrot;

fn main() {
    println!("Hello, world!");

    let input = "101001";
    println!("{}", i32::from_str_radix(input, 2).unwrap());

    misc::linear_regression::run();

    rustbook::types::run();

    rustbook::overflow::run(127);
    rustbook::overflow::run(128);

    rustbook::fibonacci::run();

    println!(
        "30C is {:.1}F",
        rustbook::temperature::celsius_to_fahrenheit(30.0)
    );
    println!(
        "80F is {:.1}C",
        rustbook::temperature::fahrenheit_to_celsius(80.0)
    );

    rustbook::twelve_days::write_song();

    mandelbrot::mandelbrot::run();
}
