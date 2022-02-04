use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
enum BoxStatus {
    Checked,
    Unchecked,
}

const GRID_LENGTH: usize = 5;
const GRID_WIDTH: usize = 5;

#[derive(Debug)]
#[allow(dead_code)]
struct GridElement {
    status: BoxStatus,
    value: usize
}

#[derive(Debug)]
#[allow(dead_code)]
struct BingoGrid {
    grid: HashMap<(usize, usize), GridElement>
}

fn create_bingo_grid(lines: Vec<&str>) -> BingoGrid {
    let mut hashmap_under_construction: HashMap<(usize, usize), GridElement> = HashMap::with_capacity(GRID_WIDTH*GRID_LENGTH);
    let mut line_num = 0usize;
    for line in lines {
        let mut col_num = 0usize;
        for value in line.split_whitespace() {
            hashmap_under_construction.insert((line_num, col_num), GridElement { status: BoxStatus::Unchecked, value: value.parse::<usize>().unwrap() });
            col_num += 1
        }
        line_num += 1
    }
    BingoGrid {
        grid: hashmap_under_construction,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let picked_values: Vec<usize> = lines[0].split(",").map(|x| str::parse::<usize>(x).unwrap()).collect();
    println!("{:?}", picked_values);

    println!("{:?}", create_bingo_grid(lines[2..7].to_vec()));
}
