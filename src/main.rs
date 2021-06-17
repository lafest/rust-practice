fn main() {
  let fahrenheit = 100.0;
  println!("fahrenheit: {} / converted: {}", fahrenheit, fahrenheit_to_celsius(fahrenheit));

  let celsius = 36.5;
  println!("celsius: {} / converted: {}", celsius, celsius_to_fahrenheit(celsius));

  let fibonacci_number = 7;
  println!("fibonacci_number: {} / fibonacci({}): {}", fibonacci_number, fibonacci_number, fibonacci(fibonacci_number));

  let rect1 = Rectangle { length: 50, width: 30 };
  println!("rect1 is {:#?}", rect1);
  println!(
    "The area of the rectangle is {} square pixels",
    area(&rect1) // 이 함수의 목적은 소유권을 가져가는것이 아닌 빌림이기 때문에 참조자를 넘겨줌
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

#[derive(Debug)] // debug포맷을 사용하고자 하면 명시적 언급이 필요
struct Rectangle {
  length: i32,
  width: i32,
}

fn area(rectangle: &Rectangle) -> i32 {
  rectangle.length * rectangle.width
}