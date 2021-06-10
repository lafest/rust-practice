fn main() {
  let fahrenheit: f64 = 100.0;
  let celsius: f64 = 36.5;
  let fibonacci_number: i64 = 7;
  println!("fahrenheit: {} / converted: {}", fahrenheit, fahrenheit_to_celsius(fahrenheit));
  println!("celsius: {} / converted: {}", celsius, celsius_to_fahrenheit(celsius));
  println!("fibonacci_number: {} / fibonacci({}): {}", fibonacci_number, fibonacci_number, fibonacci(fibonacci_number))
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
  celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
  (fahrenheit - 32.0) / 1.8
}

fn fibonacci(n: i64) -> i64 {
  if n == 0 {
    0
  } else if n == 1 {
    1
  } else {
    fibonacci(n - 1) + fibonacci(n - 2)
  }
}