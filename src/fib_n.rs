// use rand::{self, Rng};
// use std::cmp::Ordering;
use std::io::stdin;

pub fn fib_n(n: i32) -> u64 {
    let mut vec: Vec<u64> = vec![0, 1, 1];

    if n <= 2 {
        return vec[(n-1) as usize];
    };

    let mut current_num: u64 = 0;

    for i in 2..n {
        current_num = vec[(i-1) as usize] + vec[i as usize];
        vec.push(current_num);
    };

    return current_num;
}

pub fn _get_fib() {
    let mut user_input = String::new();
    let n: i32;

    println!("Please, input n-fib num: ");
    stdin().read_line(&mut user_input).expect("Is not a valid input");

    n = user_input.trim().parse().expect("Is not a valid num");

    let fibbonaci: u64 = fib_n(n);
    println!("{fibbonaci}");
}
