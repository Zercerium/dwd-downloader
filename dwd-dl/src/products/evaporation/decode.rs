use std::str::Lines;

use crate::util::point::Point;

// TODO check header, extract no_data values => use an enum
pub fn decode(data: &str, coordinates: &[Point<usize>]) -> Vec<f32> {
    let lines = data.lines();
    extract_points(coordinates, lines)
}

pub fn extract_points(coordinates: &[Point<usize>], mut lines: Lines<'_>) -> Vec<f32> {
    let coordinates = sort_coordinates_y(coordinates);
    // offset top right corner (1,1) but index starts at 0
    let coordinates = coordinates
        .iter()
        .map(|p| Point::new(p.x - 1, p.y - 1))
        .collect::<Vec<_>>();

    let mut res = Vec::new();
    let mut current_coord = Point::new(0, 0);
    let mut current_line = lines.next().unwrap().split_whitespace();
    for coord in coordinates {
        if coord.y != current_coord.y {
            current_line = lines
                .nth(coord.y - current_coord.y - 1)
                .unwrap()
                .split_whitespace();
            current_coord = Point::new(0, coord.y);
        }
        let el = current_line.nth(coord.x - current_coord.x);
        current_coord.x = coord.x + 1;
        res.push(el.unwrap().parse::<f32>().unwrap() / 10.);
    }

    res
}

pub fn sort_coordinates_y<T>(coordinates: &[Point<T>]) -> Vec<Point<T>>
where
    T: Ord + Copy,
{
    let mut res = coordinates.to_vec();
    res.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    res
}

pub fn sort_coordinates_x<T>(coordinates: &[Point<T>]) -> Vec<Point<T>>
where
    T: Ord + Copy,
{
    let mut res = coordinates.to_vec();
    res.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_points() {
        let coordinates = vec![
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(4, 4),
        ];
        let data = "1 2 3 4\n5 6 7 8\n9 10 11 12\n13 14 15 16";
        let res = extract_points(&coordinates, data.lines());
        assert_eq!(vec![0.1, 0.6, 0.7, 1.6], res);
    }
}
