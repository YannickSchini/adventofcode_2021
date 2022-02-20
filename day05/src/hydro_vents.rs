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
        else {
            return Err("Non horizontal or vertical vent line".to_string())
        }
        Ok(res)
    }
}

#[test]
fn vent_line_test() {
    let test_vent_line_horizontal = VentLine { start_point: Point { x_coord: 1, y_coord: 3}, end_point: Point{ x_coord: 2, y_coord: 3} };
    let expected_result_horizontal: Vec<Point> = vec![Point { x_coord: 1, y_coord: 3}, Point { x_coord: 2, y_coord: 3}];
    let test_vent_line_vertical = VentLine { start_point: Point { x_coord: 1, y_coord: 5}, end_point: Point{ x_coord: 1, y_coord: 3} };
    let expected_result_vertical: Vec<Point> = vec![Point { x_coord: 1, y_coord: 3}, Point { x_coord: 1, y_coord: 4}, Point { x_coord: 1, y_coord: 5}];
    assert_eq!(test_vent_line_horizontal.get_vents().unwrap(), expected_result_horizontal);
    assert_eq!(test_vent_line_vertical.get_vents().unwrap(), expected_result_vertical);
}
