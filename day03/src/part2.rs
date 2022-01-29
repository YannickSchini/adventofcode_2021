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
        let filtered_lines = vector_function(lines, index);
        if filtered_lines.len() <= 1 {
            println!("Last line: {:?}", filtered_lines);
            break
        }
        index += 1;
        lines = filtered_lines;
    }
}

fn vector_function(mut param: Vec<&str>, index: usize) -> Vec<&str> {
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
        ones
    } else if zeroes.len() > ones.len() {
        zeroes
    }
    else {
        ones
    }
}
