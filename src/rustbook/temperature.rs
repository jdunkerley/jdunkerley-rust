pub fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * (9.0 / 5.0)) + 32.0
}

pub fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}
