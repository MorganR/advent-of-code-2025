use std::collections::HashSet;
use std::fmt::{Debug, Display};

use nalgebra::{Point2, point};

use crate::utils::input::Error;

type TilePoint = Point2<usize>;

fn rectangle_area(a: &TilePoint, b: &TilePoint) -> i64 {
    let width = a.x.max(b.x) - b.x.min(a.x);
    let height = a.y.max(b.y) - b.y.min(a.y);
    // Rectangles are inclusive, so add 1.
    ((width as i64) + 1) * ((height as i64) + 1)
}

fn largest_rectangle_with_corners(points: &[TilePoint]) -> [TilePoint; 2] {
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

/// Tile colors.
#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Other,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::Red => "#",
            Self::Green => "X",
            Self::Other => ".",
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// The direction of a horizontal or vertical line.
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_start_to_end(start: &TilePoint, end: &TilePoint) -> Self {
        if start.x == end.x {
            if start.y < end.y {
                Self::Down
            } else {
                Self::Up
            }
        } else {
            if start.x < end.x {
                Self::Right
            } else {
                Self::Left
            }
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Self::Left | Self::Right => true,
            _ => false,
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }

    fn is_right_of(&self, other: Direction) -> bool {
        match (*self, other) {
            (Self::Left, Self::Down)
            | (Self::Up, Self::Left)
            | (Self::Right, Self::Up)
            | (Self::Down, Self::Right) => true,
            _ => false,
        }
    }

    fn is_left_of(&self, other: Direction) -> bool {
        match (*self, other) {
            (Self::Down, Self::Left)
            | (Self::Right, Self::Down)
            | (Self::Up, Self::Right)
            | (Self::Left, Self::Up) => true,
            _ => false,
        }
    }

    fn rotate(&self, rotation: Rotation) -> Self {
        match (*self, rotation) {
            (Self::Left, Rotation::Clockwise) => Self::Up,
            (Self::Up, Rotation::Clockwise) => Self::Right,
            (Self::Right, Rotation::Clockwise) => Self::Down,
            (Self::Down, Rotation::Clockwise) => Self::Left,
            (Self::Left, Rotation::CounterClockwise) => Self::Down,
            (Self::Down, Rotation::CounterClockwise) => Self::Right,
            (Self::Right, Rotation::CounterClockwise) => Self::Up,
            (Self::Up, Rotation::CounterClockwise) => Self::Left,
        }
    }

    /// Create a new point by shifting in the given direciton, avoiding going below zero on either axis.
    fn shift_point(&self, point: &TilePoint) -> Option<TilePoint> {
        match *self {
            Self::Left => {
                if point.x == 0 {
                    None
                } else {
                    Some(point![point.x - 1, point.y])
                }
            }
            Self::Right => Some(point![point.x + 1, point.y]),
            Self::Up => {
                if point.y == 0 {
                    None
                } else {
                    Some(point![point.x, point.y - 1])
                }
            }
            Self::Down => Some(point![point.x, point.y + 1]),
        }
    }
}

/// A polygon made only of square angled (90 degree) edges.
struct SquarePolygon {
    offset: TilePoint,
    grid: Vec<Vec<Color>>,
}

impl SquarePolygon {
    fn from_connected_points(points: &[TilePoint]) -> Self {
        let min_x: usize = points.iter().map(|p| p.x).min().unwrap_or(0);
        let max_x: usize = points.iter().map(|p| p.x).max().unwrap_or(0);
        let min_y: usize = points.iter().map(|p| p.y).min().unwrap_or(0);
        let max_y: usize = points.iter().map(|p| p.y).max().unwrap_or(0);
        let offset = TilePoint::new(min_x, min_y);
        let mut grid = Vec::with_capacity(max_y - min_y + 1);
        for _y in 0..=(max_y - min_y) {
            let mut row = Vec::with_capacity(max_x - min_x + 1);
            for _x in 0..=(max_x - min_x) {
                row.push(Color::Other);
            }
            grid.push(row);
        }

        // Draw the perimeter and determine the direction of each line.
        // The matching index in points corresponds to the end of the line.
        let mut line_directions = Vec::with_capacity(points.len());
        let zero = point![0, 0];
        let line_ends: Vec<TilePoint> = points
            .iter()
            .map(|p| point![p.x - offset.x, p.y - offset.y])
            .collect();
        let mut line_starts = Vec::with_capacity(line_ends.len());
        line_starts.push(line_ends.last().unwrap_or(&zero));
        line_starts.extend(line_ends.iter().take(line_ends.len() - 1));
        for (start, end) in line_starts.iter().zip(line_ends.iter()) {
            let direction = Direction::from_start_to_end(start, end);
            line_directions.push(direction);

            for y in start.y.min(end.y)..=start.y.max(end.y) {
                let row = grid.get_mut(y).unwrap();
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    let this_point = point![x, y];
                    if this_point == **start || this_point == *end {
                        row[x] = Color::Red;
                    } else {
                        row[x] = Color::Green;
                    }
                }
            }
        }

        // Prepare the queue of the first line of inner points to fill.
        let mut filled = HashSet::new();
        let mut cw_fill = Vec::new();
        let mut ccw_fill = Vec::new();
        for (i, (start, end)) in line_starts.iter().zip(line_ends.iter()).enumerate() {
            let line_direction = line_directions[i];
            let cw_dir = line_direction.rotate(Rotation::Clockwise);
            let ccw_dir = line_direction.rotate(Rotation::CounterClockwise);

            for y in start.y.min(end.y)..=start.y.max(end.y) {
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    if let Some(fill_point) = cw_dir.shift_point(&point![x, y]) {
                        if let Some(row) = grid.get(fill_point.y) {
                            if let Some(tile) = row.get(fill_point.x) {
                                if *tile == Color::Other {
                                    cw_fill.push(fill_point);
                                }
                            }
                        }
                    }
                    if let Some(fill_point) = ccw_dir.shift_point(&point![x, y]) {
                        if let Some(row) = grid.get(fill_point.y) {
                            if let Some(tile) = row.get(fill_point.x) {
                                if *tile == Color::Other {
                                    ccw_fill.push(fill_point);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Fill the inside with green. Points in to_fill are already shifted by the offset.
        // Try clockwise first, fail if we hit an edge.
        let mut hit_edge = false;
        while !cw_fill.is_empty() {
            let point = cw_fill.pop().unwrap();
            if grid[point.y][point.x] != Color::Other {
                continue;
            }
            if point.y == 0 || point.y == grid.len() - 1 {
                hit_edge = true;
                break;
            }
            let row = grid.get_mut(point.y).unwrap();
            if point.x == 0 || point.x == row.len() - 1 {
                hit_edge = true;
                break;
            }
            grid[point.y][point.x] = Color::Green;
            filled.insert(point);
            for y in (point.y - 1)..=(point.y + 1) {
                let row = grid.get(y).unwrap();
                for x in (point.x - 1)..=(point.x + 1) {
                    if row[x] == Color::Other {
                        cw_fill.push(point![x, y]);
                    }
                }
            }
        }

        if !hit_edge {
            log::info!("Filled polygon clockwise");
            return Self { offset, grid };
        }
        log::info!("Filling polygon clockwise failed; trying counter clockwise");

        for point in filled {
            grid[point.y][point.x] = Color::Other;
        }

        while !ccw_fill.is_empty() {
            let point = ccw_fill.pop().unwrap();
            if grid[point.y][point.x] != Color::Other {
                continue;
            }
            grid[point.y][point.x] = Color::Green;
            for y in (point.y - 1)..=(point.y + 1) {
                let row = grid.get(y).unwrap();
                for x in (point.x - 1)..=(point.x + 1) {
                    if row[x] == Color::Other {
                        ccw_fill.push(point![x, y]);
                    }
                }
            }
        }

        Self { offset, grid }
    }

    fn color_at(&self, point: &TilePoint) -> Color {
        if point.x < self.offset.x || point.y < self.offset.y {
            return Color::Other;
        }

        *self
            .grid
            .get(point.y - self.offset.y)
            .and_then(|row| row.get(point.x - self.offset.x))
            .unwrap_or(&Color::Other)
    }

    fn is_inside_shape(&self, point: &TilePoint) -> bool {
        self.color_at(point) != Color::Other
    }
}

impl Display for SquarePolygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Offset: ({},{})\n", self.offset.x, self.offset.y)?;
        for row in &self.grid {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn all_perimeter_inside_shape(a: &TilePoint, b: &TilePoint, shape: &SquarePolygon) -> bool {
    // Iterate y separately since these can follow a single vector (better CPU caching).
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);
    for x in min_x..=max_x {
        if !shape.is_inside_shape(&point![x, a.y]) {
            return false;
        }
    }
    for x in min_x..=max_x {
        if !shape.is_inside_shape(&point![x, b.y]) {
            return false;
        }
    }

    // Iterate x together since these have to jump rows anyway.
    for y in min_y..=max_y {
        if !shape.is_inside_shape(&point![a.x, y]) || !shape.is_inside_shape(&point![b.x, y]) {
            return false;
        }
    }

    true
}

fn largest_green_red_rectangle_with_corners(points: &[TilePoint]) -> [TilePoint; 2] {
    let shape = SquarePolygon::from_connected_points(points);

    let mut largest_area = 0;
    let mut largest_rectangle = [point![0, 0], point![0, 0]];

    for i in 0..points.len() {
        let a = points[i];
        for j in (i + 1)..points.len() {
            let b = points[j];

            let area = rectangle_area(&a, &b);
            if area > largest_area && all_perimeter_inside_shape(&a, &b, &shape) {
                largest_rectangle = [a, b];
                largest_area = area;
            }
        }
    }

    largest_rectangle
}

fn parse_point(input: &str) -> Result<TilePoint, Error> {
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
        x.parse::<usize>()
            .map_err(|err| Error::ParseError(format!("could not parse number {}; {:?}", x, err)))?,
        y.parse::<usize>()
            .map_err(|err| Error::ParseError(format!("could not parse number {}; {:?}", y, err)))?
    ])
}

fn parse_points(input: &str) -> Result<Vec<TilePoint>, Error> {
    input.lines().map(|line| parse_point(line)).collect()
}

pub fn part1(input: &str) -> Result<i64, Error> {
    let points = parse_points(input)?;
    let largest_rectangle = largest_rectangle_with_corners(&points);
    Ok(rectangle_area(&largest_rectangle[0], &largest_rectangle[1]))
}

pub fn part2(input: &str) -> Result<i64, Error> {
    let points = parse_points(input)?;
    let largest_rectangle = largest_green_red_rectangle_with_corners(&points);
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

    #[test]
    fn test_polygon_construction() {
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
        let shape = SquarePolygon::from_connected_points(&points);

        assert_eq!(
            format!("{}", &shape),
            "Offset: (2,1)
.....#XXX#
.....XXXXX
#XXXX#XXXX
XXXXXXXXXX
#XXXXXX#XX
.......XXX
.......#X#
"
        );
    }

    #[test]
    fn test_largest_green_red_rectangle_with_corners() {
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
        let [a, b] = largest_green_red_rectangle_with_corners(&points);

        assert_eq!(rectangle_area(&a, &b), 24);
    }
}
