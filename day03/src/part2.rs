use std::str;
use std::usize;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    let mut lines: Vec<&str> = contents.lines().collect();
    let mut index: usize = 0;
    loop {
        let filtered_lines = vector_function(lines, index, "oxygen");
        if filtered_lines.len() <= 1 {
            println!("Oxygen rate: {}", filtered_lines[0]);
            let oxygen_rate = usize::from_str_radix(filtered_lines[0], 2).unwrap();
            println!("{}", oxygen_rate);
            break
        }
        index += 1;
        lines = filtered_lines;
    }

    let mut lines: Vec<&str> = contents.lines().collect();
    let mut index: usize = 0;
    loop {
        let filtered_lines = vector_function(lines, index, "co2");
        if filtered_lines.len() <= 1 {
            println!("CO2 rate: {}", filtered_lines[0]);
            let co2_rate = usize::from_str_radix(filtered_lines[0], 2).unwrap();
            println!("{}", co2_rate);
            break
        }
        index += 1;
        lines = filtered_lines;
    }
}


fn vector_function<'a>(mut param: Vec<&'a str>, index: usize, rate: &'a str) -> Vec<&'a str> {
    param.sort();
    let mut cutoff_point: usize = 0;
    for vector_element in &param {
        if vector_element.chars().nth(index).unwrap() == '0' {
            cutoff_point += 1;
        }
    }

    let zeroes = param[..cutoff_point].to_vec();
    let ones = param[cutoff_point..].to_vec();

    if zeroes.len() == ones.len() {
        if rate.eq("oxygen") {
            return ones;
        } else {
            return zeroes;
        }
    } else if zeroes.len() > ones.len() {
        if rate.eq("oxygen") {
            return zeroes;
        } else {
            return ones;
        }
    }
    else {
        if rate.eq("oxygen") {
            return ones;
        } else {
            return zeroes;
        }
    }
}
