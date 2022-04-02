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

    let mut fish_colony: LanternfishColony = LanternfishColony::from_str(&input).unwrap();
    for day in 1..81 {
        println!("Fish colony at day {}: {:?}", day, fish_colony);
        fish_colony = fish_colony.evolve_pop_by_one_day()
    }
    println!("Final population = {}", fish_colony.get_size_of_population())
}
