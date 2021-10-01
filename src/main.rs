mod misc;
mod rustbook;

fn main() {
    println!("Hello, world!");

    misc::linear_regression::run();
    rustbook::overflow::overflow();
}
