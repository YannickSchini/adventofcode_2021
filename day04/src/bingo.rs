use crate::GRID_LENGTH;
use crate::GRID_WIDTH;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BoxStatus {
    Checked,
    Unchecked,
}

#[derive(Debug)]
pub struct GridElement {
    pub status: BoxStatus,
    pub value: usize
}

impl GridElement {
    fn check_grid_element(&mut self) {
        self.status = BoxStatus::Checked
    }
}

#[derive(Debug)]
pub struct BingoGrid {
    pub grid: HashMap<(usize, usize), GridElement>
}

impl BingoGrid {
    pub fn check_a_number(&mut self, picked_number: usize) {
        // let key = &self.grid.iter().find_map(|(key, val)| if val.value == picked_number { Some(key) } else { None });
        let key = crate::find_key_for_value(&self.grid, picked_number);
        self.grid.get_mut(&key.unwrap()).unwrap().check_grid_element()
    }

    pub fn has_grid_won(&self) -> bool {
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

    pub fn calculate_sum_of_unmarked(&self) -> usize {
        let mut sum = 0usize;
        for (_, grid_element) in self.grid.iter() {
            if grid_element.status == BoxStatus::Unchecked {
                sum += grid_element.value;
            }
        }
        sum
    }
}

pub fn create_bingo_grid(lines: Vec<&str>) -> BingoGrid {
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
