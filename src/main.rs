// using crates
use std::io;

// Constants
const ONE: f64 = 1.0;
const TWO: f64 = 2.0;

// Main loop
fn main() {
    println!("ルートを近似したい数字と精度を半角スペースで区切って入れてください");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("faild String");
    let input_vec: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect("Faild convarted"))
        .collect();
    let rooted: f64 = dichotomy(input_vec[0], input_vec[1] as i32);
    println!("Root of input: {}", rooted);
}

// Functions
fn dichotomy(num: f64, accuracy: i32) -> f64 {
    // Variables
    let mut count: i32 = 0;
    let mut minimum: f64 = ONE;
    let mut maximum: f64 = num;
    let mut median: f64;
    let mut powerd_med: f64;
    let mut root: f64 = 0.0;
    while count < accuracy {
        median = (minimum + maximum) / TWO;
        powerd_med = median * median;
        if num < powerd_med {
            maximum = median;
            root = median;
            println!("{}", root);
            count += 1;
        }
        else {
            minimum = median;
            root = median;
            println!("{}", root);
            count += 1;
        }
    }
    root 
}