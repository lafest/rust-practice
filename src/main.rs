fn main() {
  let fahrenheit = 100.0;
  println!("fahrenheit: {} / converted: {}", fahrenheit, fahrenheit_to_celsius(fahrenheit));

  let celsius = 36.5;
  println!("celsius: {} / converted: {}", celsius, celsius_to_fahrenheit(celsius));

  let fibonacci_number = 7;
  println!("fibonacci_number: {} / fibonacci({}): {}", fibonacci_number, fibonacci_number, fibonacci(fibonacci_number));

  let rect1 = (50, 30);
  println!(
    "The area of the rectangle is {} square pixels",
    area(rect1)
  );
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

fn area(dimensions: (i32, i32)) -> i32 {
  dimensions.0 * dimensions.1
}