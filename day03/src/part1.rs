use std::env;
use std::fs;
use std::str;
use std::usize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut gamma_rate_string: String = "".to_string();
    let mut epsilon_rate_string: String = "".to_string();

    for bit in 0..lines[0].len() {
        let mut bit_sum = 0usize;
        for line in &lines {
            bit_sum = bit_sum + line.chars().nth(bit).unwrap().to_digit(10).unwrap() as usize
        }
        if bit_sum > lines.len()/2 {
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
        } else {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        }
    }
    println!("Gamma_rate_string: {}", gamma_rate_string);
    let gamma_rate = usize::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_string, 2).unwrap();
    println!("Gamma rate in decimal: {}", gamma_rate);
    println!("Epsilon rate in decimal: {}", epsilon_rate);
    println!("Solution: {}", gamma_rate * epsilon_rate)
}
