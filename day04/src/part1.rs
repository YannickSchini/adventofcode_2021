use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;

#[derive(Debug)]
enum BoxStatus {
    Checked,
    Unchecked,
}

const GRID_LENGTH: usize = 5;
const GRID_WIDTH: usize = 5;

#[derive(Debug)]
struct GridElement {
    status: BoxStatus,
    value: usize
}

impl GridElement {
    fn check_grid_element(&mut self) {
        self.status = BoxStatus::Checked
    }
}

#[derive(Debug)]
struct BingoGrid {
    grid: HashMap<(usize, usize), GridElement>
}

impl BingoGrid {
    fn check_a_number(&mut self, picked_number: usize) {
        // let key = &self.grid.iter().find_map(|(key, val)| if val.value == picked_number { Some(key) } else { None });
        let key = find_key_for_value(&self.grid, picked_number);
        self.grid.get_mut(&key.unwrap()).unwrap().check_grid_element()
    }
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
    let bingo_grid = BingoGrid {
        grid: hashmap_under_construction,
    };
    bingo_grid
}

fn find_key_for_value<'a>(map: &'a HashMap<(usize, usize), GridElement>, value: usize) -> Option<(usize, usize)> {
    map.iter()
        .find_map(|(key, val)| if val.value == value { Some(key.clone()) } else { None })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    let picked_values: Vec<usize> = lines[0].split(",").map(|x| str::parse::<usize>(x).unwrap()).collect();
    println!("{:?}", picked_values);

    let mut bingo_grids: Vec<BingoGrid> = Vec::new();
    let mut iterator = 0;
    loop {
        bingo_grids.push(create_bingo_grid(lines[2+iterator+(GRID_LENGTH*iterator)..2+iterator+(GRID_LENGTH*(iterator+1))].to_vec()));
        iterator += 1;
        if 2+iterator+(GRID_LENGTH*iterator) >= lines.len() {
            break
        }
    }
    for bingo in &bingo_grids {
        println!("{:?}", bingo);
    }
    let mut test_bingo_grid = create_bingo_grid(lines[2..7].to_vec());
    println!("{:?}",  test_bingo_grid);
    test_bingo_grid.check_a_number(22);
    println!("{:?}",  test_bingo_grid);
}
