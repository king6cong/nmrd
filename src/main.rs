extern crate rustc_demangle;
use std::io;
use rustc_demangle::demangle;

fn process() {
    let mut input = String::new();
    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            break;
        }
        let mut symbol = input[18..].trim().to_owned();
        symbol = demangle(&symbol).to_string();
        println!("{} {}", &input[..18].to_owned(), symbol);
        input.clear();
    }
}

fn main() {
    process();
}
