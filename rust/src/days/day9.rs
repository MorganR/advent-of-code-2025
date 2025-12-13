use nalgebra::{Point2, point};

use crate::utils::input::Error;

fn rectangle_area(a: &Point2<i64>, b: &Point2<i64>) -> i64 {
    let width = a.x - b.x;
    let height = a.y - b.y;
    // Rectangles are inclusive, so add 1.
    (width.abs() + 1) * (height.abs() + 1)
}

fn largest_rectangle_with_corners(points: &[Point2<i64>]) -> [Point2<i64>; 2] {
    let mut largest_area = 0;
    let mut largest_rectangle = [point![0, 0], point![0, 0]];

    for i in 0..points.len() {
        let a = points[i];
        for j in (i + 1)..points.len() {
            let b = points[j];
            let area = rectangle_area(&a, &b);
            if area > largest_area {
                largest_rectangle = [a, b];
                largest_area = area;
            }
        }
    }

    largest_rectangle
}

fn parse_point(input: &str) -> Result<Point2<i64>, Error> {
    let numbers: Vec<&str> = input.split(',').collect();
    if numbers.len() != 2 {
        return Err(Error::ParseError(format!(
            "points must have 2 numbers, found {} in {}",
            numbers.len(),
            input
        )));
    }
    let x = numbers[0];
    let y = numbers[1];
    Ok(point![
        x.parse::<i64>()
            .map_err(|err| Error::ParseError(format!("could not parse number {}; {:?}", x, err)))?,
        y.parse::<i64>()
            .map_err(|err| Error::ParseError(format!("could not parse number {}; {:?}", y, err)))?
    ])
}

fn parse_points(input: &str) -> Result<Vec<Point2<i64>>, Error> {
    input.lines().map(|line| parse_point(line)).collect()
}

pub fn part1(input: &str) -> Result<i64, Error> {
    let points = parse_points(input)?;
    let largest_rectangle = largest_rectangle_with_corners(&points);
    Ok(rectangle_area(&largest_rectangle[0], &largest_rectangle[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_with_corners() {
        let points = [
            point!(7, 1),
            point!(11, 1),
            point!(11, 7),
            point!(9, 7),
            point!(9, 5),
            point!(2, 5),
            point!(2, 3),
            point!(7, 3),
        ];
        let [a, b] = largest_rectangle_with_corners(&points);

        assert_eq!(rectangle_area(&a, &b), 50);
    }
}
