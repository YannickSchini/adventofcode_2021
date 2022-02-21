use std::fs;
use std::env;
use std::str::FromStr;

mod lanternfish_colony;
use lanternfish_colony::LanternfishColony;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input: &str = contents.lines().collect::<Vec<&str>>()[0];

    let fish_pop_1 = LanternfishColony::from_str(&input).unwrap();

    println!("{:?}", fish_pop_1);
    let fish_pop_2 = fish_pop_1.evolve_pop_by_one_day();
    println!("{:?}", fish_pop_2);
}
