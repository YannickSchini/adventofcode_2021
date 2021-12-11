use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut counter = 0u16;
    for index in 0..(lines.len() - 4) {
        if (lines[index].parse::<i16>().unwrap() + lines[index + 1].parse::<i16>().unwrap() + lines[index + 2].parse::<i16>().unwrap())
            - (lines[index + 1].parse::<i16>().unwrap() + lines[index + 2].parse::<i16>().unwrap() + lines[index + 3].parse::<i16>().unwrap()) < 0 {
            counter += 1;
        }
    }
    println!("{}", counter)
}
