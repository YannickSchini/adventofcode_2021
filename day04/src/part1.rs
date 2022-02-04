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
    let mut bingo_grid = BingoGrid {
        grid: hashmap_under_construction,
    };
    bingo_grid
}

#[allow(dead_code)]
impl BingoGrid {
    fn check_a_number(&mut self, picked_number: usize) {
        let key = &self.grid.iter().find_map(|(key, val)| if val.value == picked_number { Some(key) } else { None });
        &mut self.grid.get_mut(key.unwrap()).unwrap().status = BoxStatus::Checked;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let picked_values: Vec<usize> = lines[0].split(",").map(|x| str::parse::<usize>(x).unwrap()).collect();
    println!("{:?}", picked_values);
    let bingo_grid_1 = create_bingo_grid(lines[2..7].to_vec());
    let bingo_grid_2 = create_bingo_grid(lines[8..13].to_vec());
    let bingo_grid_3 = create_bingo_grid(lines[14..19].to_vec());

    println!("{:?}", bingo_grid_1);
    println!("{:?}", bingo_grid_2);
    println!("{:?}", bingo_grid_3);

}
