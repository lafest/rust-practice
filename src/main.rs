fn main() {
  let fahrenheit: f64 = 100.0;
  let celsius: f64 = 36.5;
  println!("fahrenheit: {} / converted: {}", fahrenheit, fahrenheit_to_celsius(fahrenheit));
  println!("celsius: {} / converted: {}", celsius, celsius_to_fahrenheit(celsius));
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
  celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
  (fahrenheit - 32.0) / 1.8
}