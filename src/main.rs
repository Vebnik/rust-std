mod temp_converter;
mod fib_n;

// use rand::{self, Rng};
// use std::cmp::Ordering;
// use std::io::stdin;

fn main() {
    let temp: f32 = temp_converter::temp_convert(
        32.0,
        temp_converter::TempUnit::Fahrenheit, 
        temp_converter::TempUnit::Celsius
    );

    let fib: u64 = fib_n::fib_n(90);

    println!("Temp: {temp}");
    println!("a 90n fib seq is: {fib}")
}
