use std::cmp::max;
use std::cmp::min;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct Point {
    pub x_coord: usize,
    pub y_coord: usize,
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct VentLine {
    pub start_point: Point,
    pub end_point: Point,
}

impl VentLine {
    pub fn get_vents(&self) -> Result<Vec<Point>, String> {
        let mut res: Vec<Point> = Vec::new();
        if &self.start_point.x_coord == &self.end_point.x_coord {
            if self.start_point.y_coord <= self.end_point.y_coord {
                for coord in self.start_point.y_coord..self.end_point.y_coord+1 {
                    res.push(Point { x_coord: self.start_point.x_coord, y_coord: coord})
                }
            }
            else {
                for coord in self.end_point.y_coord..self.start_point.y_coord+1 {
                    res.push(Point { x_coord: self.start_point.x_coord, y_coord: coord})
                }
            }
        }
        else if &self.start_point.y_coord == &self.end_point.y_coord {
            if self.start_point.x_coord <= self.end_point.x_coord {
                for coord in self.start_point.x_coord..self.end_point.x_coord+1 {
                    res.push(Point { x_coord: coord, y_coord: self.start_point.y_coord})
                }
            }
            else {
                for coord in self.end_point.x_coord..self.start_point.x_coord+1 {
                    res.push(Point { x_coord: coord, y_coord: self.start_point.y_coord})
                }
            }
        }
        else if self.start_point.x_coord <= self.end_point.x_coord {
            if self.start_point.y_coord <= self.end_point.y_coord {
                let mut i:usize = 0;
                loop {
                    let new_point = Point { x_coord: self.start_point.x_coord + i, y_coord: self.start_point.y_coord + i};
                    if new_point == self.end_point {
                        res.push(new_point);
                        break
                    }
                    res.push(new_point);
                    i += 1;
                }
            }
            else {
                let mut i:usize = 0;
                loop {
                    let new_point = Point { x_coord: self.start_point.x_coord + i, y_coord: self.start_point.y_coord - i};
                    if new_point == self.end_point {
                        res.push(new_point);
                        break
                    }
                    res.push(new_point);
                    i += 1;
                }
            }
        }
        else {
            if self.start_point.y_coord <= self.end_point.y_coord {
                let mut i:usize = 0;
                loop {
                    let new_point = Point { x_coord: self.start_point.x_coord - i, y_coord: self.start_point.y_coord + i};
                    if new_point == self.end_point {
                        res.push(new_point);
                        break
                    }
                    res.push(new_point);
                    i += 1;
                }
            }
            else {
                let mut i:usize = 0;
                loop {
                    let new_point = Point { x_coord: self.start_point.x_coord - i, y_coord: self.start_point.y_coord - i};
                    if new_point == self.end_point {
                        res.push(new_point);
                        break
                    }
                    res.push(new_point);
                    i += 1;
                }
            }
        }
        Ok(res)
    }
}

#[test]
fn vent_line_test_horizontal_and_vertical() {
    let test_vent_line_horizontal = VentLine { start_point: Point { x_coord: 1, y_coord: 3}, end_point: Point{ x_coord: 2, y_coord: 3} };
    let expected_result_horizontal: Vec<Point> = vec![Point { x_coord: 1, y_coord: 3}, Point { x_coord: 2, y_coord: 3}];
    let test_vent_line_vertical = VentLine { start_point: Point { x_coord: 1, y_coord: 5}, end_point: Point{ x_coord: 1, y_coord: 3} };
    let expected_result_vertical: Vec<Point> = vec![Point { x_coord: 1, y_coord: 3}, Point { x_coord: 1, y_coord: 4}, Point { x_coord: 1, y_coord: 5}];
    assert_eq!(test_vent_line_horizontal.get_vents().unwrap(), expected_result_horizontal);
    assert_eq!(test_vent_line_vertical.get_vents().unwrap(), expected_result_vertical);
}

#[test]
fn vent_line_test_diagonal() {
    let test_vent_line_diagonal_1 = VentLine { start_point: Point { x_coord: 1, y_coord: 3}, end_point: Point{ x_coord: 3, y_coord: 5} };
    let test_vent_line_diagonal_2 = VentLine { start_point: Point { x_coord: 3, y_coord: 5}, end_point: Point{ x_coord: 1, y_coord: 3} };
    let test_vent_line_diagonal_3 = VentLine { start_point: Point { x_coord: 3, y_coord: 1}, end_point: Point{ x_coord: 1, y_coord: 3} };
    let test_vent_line_diagonal_4 = VentLine { start_point: Point { x_coord: 1, y_coord: 3}, end_point: Point{ x_coord: 3, y_coord: 1} };
    let expect_result_1: Vec<Point> = vec![Point { x_coord: 1, y_coord: 3}, Point { x_coord: 2, y_coord: 4}, Point { x_coord: 3, y_coord: 5 }];
    let expect_result_2: Vec<Point> = vec![Point { x_coord: 3, y_coord: 5}, Point { x_coord: 2, y_coord: 4}, Point { x_coord: 1, y_coord: 3 }];
    let expect_result_3: Vec<Point> = vec![Point { x_coord: 3, y_coord: 1}, Point { x_coord: 2, y_coord: 2}, Point { x_coord: 1, y_coord: 3 }];
    let expect_result_4: Vec<Point> = vec![Point { x_coord: 1, y_coord: 3}, Point { x_coord: 2, y_coord: 2}, Point { x_coord: 3, y_coord: 1 }];
    assert_eq!(test_vent_line_diagonal_1.get_vents().unwrap(), expect_result_1);
    assert_eq!(test_vent_line_diagonal_2.get_vents().unwrap(), expect_result_2);
    assert_eq!(test_vent_line_diagonal_3.get_vents().unwrap(), expect_result_3);
    assert_eq!(test_vent_line_diagonal_4.get_vents().unwrap(), expect_result_4);

}
