use std::env;
use std::fs;
use std::str;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
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

    fn has_grid_won(&self) -> bool {
        'cols: for index_i in 0..GRID_WIDTH {
            for index_j in 0..GRID_LENGTH {
                if self.grid.get(&(index_j, index_i)).unwrap().status == BoxStatus::Unchecked {
                    continue 'cols;
                }
            }
            return true;
        }
        'lines: for index_i in 0..GRID_WIDTH {
            for index_j in 0..GRID_LENGTH {
                if self.grid.get(&(index_i, index_j)).unwrap().status == BoxStatus::Unchecked {
                    continue 'lines;
                }
            }
            return true;
        }
        false
    }

    fn calculate_sum_of_unmarked(&self) -> usize {
        let mut sum = 0usize;
        for (_, grid_element) in self.grid.iter() {
            if grid_element.status == BoxStatus::Unchecked {
                sum += grid_element.value;
            }
        }
        sum
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
    let mut test_bingo_grid = create_bingo_grid(lines[14..19].to_vec());
    println!("{:?}",  test_bingo_grid);
    println!("{}", test_bingo_grid.has_grid_won());
    test_bingo_grid.check_a_number(7);
    test_bingo_grid.check_a_number(4);
    test_bingo_grid.check_a_number(9);
    test_bingo_grid.check_a_number(5);
    test_bingo_grid.check_a_number(11);
    test_bingo_grid.check_a_number(17);
    test_bingo_grid.check_a_number(23);
    test_bingo_grid.check_a_number(2);
    test_bingo_grid.check_a_number(0);
    test_bingo_grid.check_a_number(14);
    test_bingo_grid.check_a_number(21);
    test_bingo_grid.check_a_number(24);
    println!("{}", test_bingo_grid.has_grid_won());
    println!("{}", test_bingo_grid.calculate_sum_of_unmarked());
}
