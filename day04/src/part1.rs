use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;

mod bingo;

const GRID_LENGTH: usize = 5;
const GRID_WIDTH: usize = 5;

fn find_key_for_value<'a>(map: &'a HashMap<(usize, usize), bingo::GridElement>, value: usize) -> Option<(usize, usize)> {
    map.iter()
        .find_map(|(key, val)| if val.value == value { Some(key.clone()) } else { None })
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    let picked_values: Vec<usize> = lines[0].split(",").map(|x| str::parse::<usize>(x).unwrap()).collect();

    let mut bingo_grids: Vec<bingo::BingoGrid> = Vec::new();
    let mut iterator = 0;
    loop {
        bingo_grids.push(bingo::create_bingo_grid(lines[2+iterator+(GRID_LENGTH*iterator)..2+iterator+(GRID_LENGTH*(iterator+1))].to_vec()));
        iterator += 1;
        if 2+iterator+(GRID_LENGTH*iterator) >= lines.len() {
            break
        }
    }

    'values: for value in picked_values {
        for bingo_grid in bingo_grids.iter_mut() {
            bingo_grid.check_a_number(value);
            if bingo_grid.has_grid_won() {
                println!("{}", bingo_grid.calculate_sum_of_unmarked()*value);
                break 'values
            }
        }
    }
}
