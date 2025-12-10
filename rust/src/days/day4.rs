use crate::utils::input::Error;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Thing {
    RollOfPaper,
    Nothing,
}

impl Thing {
    fn parse(c: char) -> Result<Thing, Error> {
        match c {
            '@' => Ok(Thing::RollOfPaper),
            '.' => Ok(Thing::Nothing),
            _ => Err(Error::ParseError(format!("invalid thing: {}", c))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Location {
        Location { x, y }
    }

    fn surrounding_locations(&self) -> Vec<Location> {
        let min_x = 1.max(self.x) - 1;
        let min_y = 1.max(self.y) - 1;
        let mut surrounds = Vec::with_capacity(8);
        for x in min_x..=(self.x + 1) {
            for y in min_y..=(self.y + 1) {
                if x == self.x && y == self.y {
                    continue;
                }
                surrounds.push(Location::new(x, y))
            }
        }
        surrounds
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Grid {
    num_rows: usize,
    num_cols: usize,
    data: Vec<Thing>,
}

impl Grid {
    fn parse(grid: &str) -> Result<Grid, Error> {
        let mut num_rows = 0;
        let mut num_cols = 0;
        let mut data = Vec::new();
        let mut first_line = true;
        for (i, line) in grid.lines().enumerate() {
            if first_line {
                num_cols = line.len();
                first_line = false;
            } else if line.len() != num_cols {
                return Err(Error::ParseError(format!(
                    "inconsistent number of columns on line {i}; expected {num_cols}, found {}",
                    line.len()
                )));
            }
            for c in line.chars() {
                data.push(Thing::parse(c)?);
            }
            num_rows += 1;
        }
        Ok(Grid {
            num_cols,
            num_rows,
            data,
        })
    }

    fn set(&mut self, location: &Location, value: Thing) {
        if let Some(index) = self.to_index(location) {
            self.data[index] = value;
        }
    }

    fn at(&self, location: &Location) -> Thing {
        let index = self.to_index(location);
        match index {
            Some(idx) => self.data[idx],
            None => Thing::Nothing,
        }
    }

    fn to_index(&self, location: &Location) -> Option<usize> {
        if location.x >= self.num_cols || location.y >= self.num_rows {
            None
        } else {
            Some(location.y * self.num_cols + location.x)
        }
    }

    fn is_accessible(&self, location: &Location) -> bool {
        if self.to_index(location).is_none() {
            return false;
        }

        let num_rolls_of_paper =
            location
                .surrounding_locations()
                .into_iter()
                .fold(0u8, |acc, l| {
                    if self.at(&l) == Thing::RollOfPaper {
                        acc + 1
                    } else {
                        acc
                    }
                });
        num_rolls_of_paper < 4
    }
}

fn find_accessible_rolls_of_paper(grid: &Grid) -> Vec<Location> {
    let mut accessible = Vec::new();
    for x in 0..grid.num_cols {
        for y in 0..grid.num_rows {
            let location = Location::new(x, y);
            if grid.at(&location) == Thing::RollOfPaper && grid.is_accessible(&location) {
                log::trace!("Found accessible roll of paper at {}", location);
                accessible.push(location);
            }
        }
    }
    accessible
}

pub fn count_accessible_rolls_of_paper(grid_str: &str) -> Result<usize, Error> {
    let grid = Grid::parse(grid_str)?;

    let accessible = find_accessible_rolls_of_paper(&grid);

    Ok(accessible.len())
}

pub fn count_total_removable_rolls_of_paper(grid_str: &str) -> Result<usize, Error> {
    let mut grid = Grid::parse(grid_str)?;

    let mut total_removed = 0;
    loop {
        let accessible = find_accessible_rolls_of_paper(&grid);
        if accessible.is_empty() {
            break;
        }
        total_removed += accessible.len();
        for l in accessible {
            grid.set(&l, Thing::Nothing);
        }
    }

    Ok(total_removed)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn parse_thing_success() {
        assert_eq!(Thing::parse('.').unwrap(), Thing::Nothing);
        assert_eq!(Thing::parse('@').unwrap(), Thing::RollOfPaper);
    }

    #[test]
    fn parse_thing_error() {
        let result = Thing::parse('?');
        assert!(result.is_err());
    }

    #[test]
    fn parse_grid() {
        let result = Grid::parse(
            "..@
...
@@@
...",
        );
        assert!(result.is_ok());

        let grid = result.unwrap();
        assert_eq!(grid.num_cols, 3);
        assert_eq!(grid.num_rows, 4);
        assert_eq!(grid.at(&Location::new(1, 0)), Thing::Nothing);
        assert_eq!(grid.at(&Location::new(2, 0)), Thing::RollOfPaper);
        assert_eq!(grid.at(&Location::new(3, 0)), Thing::Nothing);
    }

    #[test]
    fn parse_invalid_grid_mismatch_num_cols() {
        let result = Grid::parse(
            "..@...
@@@",
        );
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_grid_invalid_thing() {
        let result = Grid::parse(
            "..?
@@@
@@@",
        );
        assert!(result.is_err());
    }

    #[test]
    fn location_surrounding_locations_all() {
        let result = Location::new(3, 3).surrounding_locations();
        let as_set: HashSet<Location> = result.into_iter().collect();
        assert_eq!(
            as_set,
            [
                Location::new(2, 2),
                Location::new(2, 3),
                Location::new(2, 4),
                Location::new(3, 2),
                Location::new(3, 4),
                Location::new(4, 2),
                Location::new(4, 3),
                Location::new(4, 4),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn location_surrounding_locations_top_left() {
        let result = Location::new(0, 0).surrounding_locations();
        let as_set: HashSet<Location> = result.into_iter().collect();
        assert_eq!(
            as_set,
            [
                Location::new(0, 1),
                Location::new(1, 0),
                Location::new(1, 1),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn count_accessible_rolls_of_paper_basic() {
        let result = count_accessible_rolls_of_paper(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn count_accessible_rolls_of_paper_no_rolls() {
        let result = count_accessible_rolls_of_paper(
            "..........
..........
..........
..........
..........
..........
..........
..........
..........
..........",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn count_total_removable_rolls_of_paper_basic() {
        let result = count_total_removable_rolls_of_paper(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 43);
    }
}
