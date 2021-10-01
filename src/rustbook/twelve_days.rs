const ORDINALS: [&str; 12] = [
    "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth",
    "Eleventh", "Twelfth",
];

const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn write_song() {
    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me:",
            ORDINALS[i]
        );

        for j in (0..i + 1).rev() {
            if j == i {
                print!("{}", capitalise(GIFTS[j]));
            } else {
                print!("And {}", GIFTS[j]);
            }
            println!("{}", if j == 0 { "." } else { ", " });
        }

        println!();
    }
}
