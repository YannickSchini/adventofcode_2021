use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut horizontal_counter = 0i32;
    let mut depth_counter = 0i32;
    let mut aim = 0i32;
    for index in 0..lines.len() {
        let mut parts = lines[index].split_whitespace();
        let part1 = parts.next().unwrap();
        let part2 = parts.next().unwrap().parse::<i32>().unwrap();
        match part1.chars().next() {
            Some('f') => {
                horizontal_counter = horizontal_counter + part2;
                depth_counter = depth_counter + aim * part2;
            },
            Some('u') => aim = aim - part2,
            Some('d') => aim = aim + part2,
            Some(_) => panic!("Unknown string"),
            None => println!("End of string"),
        };
    }
    println!("final score {}", horizontal_counter*depth_counter)
}
