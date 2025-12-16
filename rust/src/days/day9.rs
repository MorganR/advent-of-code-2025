use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::os::unix::net::Incoming;

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

    fn from_line_tuple(line: &(TilePoint, TilePoint)) -> Self {
        return Self::from_start_to_end(&line.0, &line.1);
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
    line_index_by_ends: HashMap<TilePoint, usize>,
    lines: Vec<(TilePoint, TilePoint)>,
    line_directions: Vec<Direction>,
    fill_rotation: Rotation,
}

impl SquarePolygon {
    fn from_connected_points(points: &[TilePoint]) -> Self {
        log::info!("Drawing polygon's perimeter");
        // Draw the perimeter and determine the direction of each line.
        // The matching index in points corresponds to the end of the line.
        let mut line_directions = Vec::with_capacity(points.len());
        let mut number_left_turns = 0;
        let mut number_right_turns = 0;
        let lines: Vec<_> = points
            .last()
            .into_iter()
            .chain(points.iter().take(points.len() - 1))
            .zip(points.iter())
            .map(|(&a, &b)| (a, b))
            .collect();
        let line_index_by_ends: HashMap<_, _> =
            points.iter().enumerate().map(|(i, &a)| (a, i)).collect();
        for (start, end) in lines.iter() {
            let direction = Direction::from_start_to_end(start, end);
            line_directions.push(direction);
            if line_directions.len() > 1 {
                let last_direction = line_directions[line_directions.len() - 2];
                if direction.is_right_of(last_direction) {
                    number_right_turns += 1;
                } else if direction.is_left_of(last_direction) {
                    number_left_turns += 1;
                }
            }
        }

        if let Some(first_direction) = line_directions.first() {
            if let Some(last_direction) = line_directions.last() {
                if first_direction.is_right_of(*last_direction) {
                    number_right_turns += 1;
                } else if first_direction.is_left_of(*last_direction) {
                    number_left_turns += 1;
                }
            }
        }

        let fill_rotation = if number_right_turns > number_left_turns {
            Rotation::Clockwise
        } else {
            Rotation::CounterClockwise
        };

        Self {
            line_index_by_ends,
            lines,
            line_directions,
            fill_rotation,
        }
    }

    fn is_rectangle_fully_inside(&self, a: &TilePoint, b: &TilePoint) -> bool {
        let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

        // If any line immediately connected to the points is coincident with the rectangle,
        // it must put the rectangle on its fill side.
        // If none are coincident, then all must put the rectangle on their fill side.
        // This can be assessed on a single corner to fully satisfy this condition.

        let horizontal_line_to_point = (point![b.x, a.y], point![a.x, a.y]);
        let vertical_line_to_point = (point![a.x, b.y], point![a.x, a.y]);
        let horizontal_line_from_point = (point![a.x, a.y], point![b.x, a.y]);
        let vertical_line_from_point = (point![a.x, a.y], point![a.x, b.y]);
        let direction_to_corner_x = Direction::from_line_tuple(&horizontal_line_to_point);
        let direction_to_corner_y = Direction::from_line_tuple(&vertical_line_to_point);
        let direction_of_rect_x = Direction::from_line_tuple(&horizontal_line_from_point);
        let direction_of_rect_y = Direction::from_line_tuple(&vertical_line_from_point);

        let incoming_idx = self.line_index_by_ends[a];
        debug_assert_eq!(&self.lines[incoming_idx].1, a);
        let incoming_direction = self.line_directions[incoming_idx];
        let incoming_fill_direction = incoming_direction.rotate(self.fill_rotation);
        let incoming_is_coincident = incoming_direction == direction_to_corner_x
            || incoming_direction == direction_to_corner_y;
        let is_fill_side_of_incoming = (incoming_direction.is_horizontal()
            && direction_of_rect_y == incoming_fill_direction)
            || (incoming_direction.is_vertical() && direction_of_rect_x == incoming_fill_direction);

        let outgoing_idx = if (incoming_idx + 1) == self.lines.len() {
            0
        } else {
            incoming_idx + 1
        };
        debug_assert_eq!(&self.lines[outgoing_idx].0, a);
        let outgoing_direction = self.line_directions[outgoing_idx];
        let outgoing_fill_direction = outgoing_direction.rotate(self.fill_rotation);
        let outgoing_is_coincident =
            outgoing_direction == direction_of_rect_x || outgoing_direction == direction_of_rect_y;
        let is_fill_side_of_outgoing = (outgoing_direction.is_horizontal()
            && direction_of_rect_y == outgoing_fill_direction)
            || (outgoing_direction.is_vertical() && direction_of_rect_x == outgoing_fill_direction);

        let has_shape_overlap_beyond_border = (incoming_is_coincident && is_fill_side_of_incoming)
            || (outgoing_is_coincident && is_fill_side_of_outgoing)
            || (is_fill_side_of_incoming && is_fill_side_of_outgoing);
        if !has_shape_overlap_beyond_border {
            return false;
        }

        // Now check if any lines cross the border of the rectangle.
        for (line, &direction) in self.lines.iter().zip(self.line_directions.iter()) {
            if direction.is_vertical() {
                if line.0.x <= min_x || line.0.x >= max_x {
                    // This line can't hit the rectangle.
                    continue;
                }
                let line_min_y = line.0.y.min(line.1.y);
                let line_max_y = line.0.y.max(line.1.y);
                if line_min_y >= max_y || line_max_y <= min_y {
                    continue;
                }
                // This line intersects the rectangle (beyond the border).
                return false;
            } else {
                if line.0.y <= min_y || line.0.y >= max_y {
                    // This line can't hit the rectangle.
                    continue;
                }
                let line_min_x = line.0.x.min(line.1.x);
                let line_max_x = line.0.x.max(line.1.x);
                if line_min_x >= max_x || line_max_x <= min_x {
                    continue;
                }
                // This line intersects the rectangle (beyond the border).
                return false;
            }
        }

        true
    }
}

fn largest_green_red_rectangle_with_corners(points: &[TilePoint]) -> [TilePoint; 2] {
    log::info!("Constructing polygon");
    let shape = SquarePolygon::from_connected_points(points);

    log::info!("Finding largest valid rectangle");
    let mut largest_area = 0;
    let mut largest_rectangle = [point![0, 0], point![0, 0]];

    for i in 0..points.len() {
        let a = points[i];
        for j in (i + 1)..points.len() {
            let b = points[j];

            let area = rectangle_area(&a, &b);
            if area > largest_area && shape.is_rectangle_fully_inside(&a, &b) {
                log::debug!(
                    "Found largest rectangle so far (area: {}) at ({}, {})",
                    area,
                    &a,
                    &b
                );
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
    log::info!("Parsing points");
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
        let points = [point!(7, 5), point!(2, 5), point!(2, 3), point!(7, 3)];
        let shape = SquarePolygon::from_connected_points(&points);

        assert_eq!(shape.fill_rotation, Rotation::Clockwise);
        assert_eq!(
            shape.lines,
            vec![
                (points[3], points[0]),
                (points[0], points[1]),
                (points[1], points[2]),
                (points[2], points[3]),
            ],
        );
        assert_eq!(
            shape.line_directions,
            vec![
                Direction::Down,
                Direction::Left,
                Direction::Up,
                Direction::Right,
            ]
        )
    }

    #[test]
    fn test_is_rectangle_fully_inside_polygon() {
        let points = vec![
            point![0, 4],  // Left arm, top corner.
            point![4, 4],  // Top left of center.
            point![4, 0],  // Top arm, left corner.
            point![8, 0],  // Top arm, right corner.
            point![8, 4],  // Top right of center.
            point![12, 4], // Right arm, top corner.
            point![12, 8], // Right arm, bottom corner.
            point![8, 8],  // Bottom right of center.
            point![8, 12], // Bottom arm, right corner.
            point![4, 12], // Bottom arm, left corner.
            point![4, 8],  // Bottom left of center.
            point![0, 8],  // Left arm, bottom corner.
        ];
        let cross = SquarePolygon::from_connected_points(&points);
        assert_eq!(cross.fill_rotation, Rotation::Clockwise);

        let fully_inside_rectangles = vec![
            (
                point![0, 4],
                point![12, 8],
                "tl to br fully connected inside",
            ),
            (
                point![0, 8],
                point![12, 4],
                "bl to tr fully connected inside",
            ),
            (
                point![0, 4],
                point![8, 8],
                "tl connect to br disconnected a",
            ),
            (
                point![0, 4],
                point![4, 8],
                "tl connect to br disconnected b",
            ),
        ];
        for (a, b, name) in fully_inside_rectangles {
            assert!(cross.is_rectangle_fully_inside(&a, &b), "{}", name);
            assert!(
                cross.is_rectangle_fully_inside(&b, &a),
                "{} - backwards",
                name
            );
        }

        let fully_outside_rectangles = vec![
            (point![0, 4], point![4, 0], "top left outside"),
            (point![8, 0], point![12, 4], "top right outside"),
            (point![12, 8], point![8, 12], "bottom right outside"),
            (point![4, 12], point![0, 8], "bottom left outside"),
        ];
        for (a, b, name) in fully_outside_rectangles {
            assert!(!cross.is_rectangle_fully_inside(&a, &b), "{}", name);
            assert!(
                !cross.is_rectangle_fully_inside(&b, &a),
                "{} - backwards",
                name
            );
        }

        let mixed_rectangles = vec![
            (point![0, 4], point![8, 0], "top left mixed"),
            (point![12, 8], point![4, 12], "bottom right mixed"),
        ];
        for (a, b, name) in mixed_rectangles {
            assert!(!cross.is_rectangle_fully_inside(&a, &b), "{}", name);
            assert!(
                !cross.is_rectangle_fully_inside(&b, &a),
                "{} - backwards",
                name
            );
        }
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
