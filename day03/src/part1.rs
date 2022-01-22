use std::char;
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
        let mut bit_vector: Vec<usize> = Vec::new();
        for line in &lines {
            bit_vector.push(line.chars().nth(bit).unwrap().to_digit(10).unwrap() as usize);
        }
        let (most_common_bit, least_common_bit) = get_most_and_least_common_bits(bit_vector);
        epsilon_rate_string.push(least_common_bit);
        gamma_rate_string.push(most_common_bit);
    }
    println!("Gamma_rate_string: {}", gamma_rate_string);
    let gamma_rate = usize::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_string, 2).unwrap();
    println!("Gamma rate in decimal: {}", gamma_rate);
    println!("Epsilon rate in decimal: {}", epsilon_rate);
    println!("Solution: {}", gamma_rate * epsilon_rate)
}

fn get_most_and_least_common_bits(arg: Vec<usize>) -> (char, char) {
    let vec_sum: usize = arg.iter().sum();
        if vec_sum > arg.len()/2 {
            ('1', '0')
        } else {
            ('0', '1')
        }
}
