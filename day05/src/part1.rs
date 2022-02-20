use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;

mod hydro_vents;
use hydro_vents::Point;

use crate::hydro_vents::VentLine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut start_point: Point;
    let mut end_point: Point;
    let mut ocean_floor: HashMap<Point, usize> = HashMap::new();
    let mut counter: usize = 0;

    for line in lines {
       let partially_parsed_vent_line = line.split(" -> ").collect::<Vec<&str>>();
       start_point = Point { x_coord: partially_parsed_vent_line[0].split(",").collect::<Vec<&str>>()[0].parse::<usize>().unwrap(),
                             y_coord: partially_parsed_vent_line[0].split(",").collect::<Vec<&str>>()[1].parse::<usize>().unwrap()};
       end_point = Point { x_coord: partially_parsed_vent_line[1].split(",").collect::<Vec<&str>>()[0].parse::<usize>().unwrap(),
                             y_coord: partially_parsed_vent_line[1].split(",").collect::<Vec<&str>>()[1].parse::<usize>().unwrap()};
       let vent_line: VentLine = VentLine { start_point, end_point };
       // println!("Vent_line: {:?}", vent_line);
       // println!("Vents: {:?}", vent_line.get_vents());

        for points in vent_line.get_vents() {
            for point in points {
                *ocean_floor.entry(point).or_insert(0) += 1;
            }
        }

    }

    for position in ocean_floor.keys() {
        if ocean_floor[position] > 1 {
            counter += 1;
        }
    }

    println!("{}", counter)
}
