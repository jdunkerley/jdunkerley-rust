mod misc;
mod rustbook;

fn main() {
    println!("Hello, world!");

    misc::linear_regression::run();

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
}
