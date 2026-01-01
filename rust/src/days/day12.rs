use core::num;
use std::{
    collections::{HashSet, VecDeque},
    path::Iter,
    ptr::null,
    str::FromStr,
};

use anyhow::Result as AnyResult;
use nalgebra::{Point2, Vector2, point};
use regex::Regex;

use crate::utils::input::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pixel {
    Gift,
    Empty,
}

impl Pixel {
    fn from_char(c: char) -> Result<Self, Error> {
        match c {
            '#' => Ok(Pixel::Gift),
            '.' => Ok(Pixel::Empty),
            _ => Err(Error::ParseError(format!("Can't parse {} as a pixel", c))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Flip {
    None,
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// A view of a shape in a specific orientation (rotation and/or flip).
struct ShapeView {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>, // Flat row-major: index = row * width + col
    rotation: Rotation,
    flip: Flip,
}

impl ShapeView {
    fn get_pixel(&self, row: usize, col: usize) -> Pixel {
        self.pixels[row * self.width + col]
    }

    /// Returns the positions of occupied pixels, in row-major order.
    fn occupied_pixels(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.pixels
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Pixel::Gift)
            .map(move |(i, _)| (i / self.width, i % self.width))
    }

    fn looks_same(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.pixels == other.pixels
    }

    fn create_flipped(&self, flip: Flip) -> Self {
        if self.flip != Flip::None {
            unimplemented!("Can't flip from an already flipped state");
        }
        let mut new_pixels = self.pixels.clone();
        match flip {
            Flip::Horizontal => {
                let mut offset = 0;
                while offset < new_pixels.len() {
                    let end = offset + self.width;
                    new_pixels[offset..end].reverse();
                    offset = end;
                }
            }
            Flip::Vertical => {
                let mut old_offset = 0;
                let mut old_end = self.width;
                let mut new_offset = new_pixels.len() - self.width;
                let mut new_end = new_pixels.len();
                while old_offset < new_pixels.len() {
                    new_pixels[new_offset..new_end]
                        .copy_from_slice(&self.pixels[old_offset..old_end]);
                    old_offset = old_end;
                    old_end = old_offset + self.width;
                    new_end = new_offset;
                    if new_offset > 0 {
                        new_offset -= self.width;
                    }
                }
            }
            _ => {
                unimplemented!("Can't flip back to 'no flip' state");
            }
        }
        Self {
            pixels: new_pixels,
            flip,
            width: self.width,
            height: self.height,
            rotation: self.rotation,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Gift {
    area: usize,
    views: Vec<ShapeView>, // Pre-computed: Rot0, Rot90, Rot180, Rot270, and flipped.
}

impl Gift {
    fn create_rotated_shape(shape: &[Vec<Pixel>], rotation: Rotation) -> ShapeView {
        let h = shape.len();
        let w = if h > 0 { shape[0].len() } else { 0 };

        let (width, height, pixels, rotation) = match rotation {
            Rotation::Rot0 => {
                // No rotation - copy as-is into flat array
                let pixels: Vec<_> = shape.iter().flat_map(|row| row.iter().copied()).collect();
                (w, h, pixels, Rotation::Rot0)
            }
            Rotation::Rot90 => {
                // Rotate 90 degrees clockwise
                // new[col][h-1-row] = old[row][col]
                let mut pixels = vec![Pixel::Empty; w * h];

                for row in 0..h {
                    for col in 0..w {
                        let new_row = col;
                        let new_col = h - 1 - row;
                        pixels[new_row * h + new_col] = shape[row][col];
                    }
                }
                (h, w, pixels, Rotation::Rot90)
            }
            Rotation::Rot180 => {
                // Rotate 180 degrees
                // new[h-1-row][w-1-col] = old[row][col]
                let mut pixels = vec![Pixel::Empty; w * h];

                for row in 0..h {
                    for col in 0..w {
                        let new_row = h - 1 - row;
                        let new_col = w - 1 - col;
                        pixels[new_row * w + new_col] = shape[row][col];
                    }
                }
                (w, h, pixels, Rotation::Rot180)
            }
            Rotation::Rot270 => {
                // Rotate 270 degrees clockwise (90 CCW)
                // new[w-1-col][row] = old[row][col]
                let mut pixels = vec![Pixel::Empty; w * h];

                for row in 0..h {
                    for col in 0..w {
                        let new_row = w - 1 - col;
                        let new_col = row;
                        pixels[new_row * h + new_col] = shape[row][col];
                    }
                }
                (h, w, pixels, Rotation::Rot270)
            }
        };

        ShapeView {
            width,
            height,
            pixels,
            rotation,
            flip: Flip::None,
        }
    }
}

impl FromStr for Gift {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shape = Vec::new();
        for line in s.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }

            let row = trimmed_line
                .chars()
                .map(|p| Pixel::from_char(p))
                .collect::<Result<Vec<_>, Error>>()?;
            shape.push(row);
        }

        let row_length = shape.iter().next().map(|r| r.len()).unwrap_or(0);
        if shape.iter().any(|r| r.len() != row_length) {
            return Err(Error::ParseError(format!(
                "could not parse '{}' as gift; all gift rows must have the same length",
                s
            )));
        }

        let area = shape
            .iter()
            .flat_map(|row| row.iter())
            .filter(|p| **p == Pixel::Gift)
            .count();

        // Pre-compute all unique views.
        let mut views = vec![Gift::create_rotated_shape(&shape, Rotation::Rot0)];

        for rotation in [Rotation::Rot90, Rotation::Rot180, Rotation::Rot270] {
            let view = Gift::create_rotated_shape(&shape, rotation);
            if !views.iter().any(|v| v.looks_same(&view)) {
                views.push(view);
            }
        }
        for i in 0..views.len() {
            for flip in [Flip::Vertical, Flip::Horizontal] {
                let flipped = (&views[i]).create_flipped(flip);
                if !views.iter().any(|v| v.looks_same(&flipped)) {
                    views.push(flipped);
                }
            }
        }

        Ok(Self { area, views })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

impl FromStr for Rectangle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return Err(Error::ParseError(format!(
                "Invalid rectangle format '{}': expected 'widthxheight'",
                s
            )));
        }

        let width = parts[0].parse::<usize>().map_err(|_| {
            Error::ParseError(format!("Invalid width '{}' in rectangle '{}'", parts[0], s))
        })?;

        let height = parts[1].parse::<usize>().map_err(|_| {
            Error::ParseError(format!(
                "Invalid height '{}' in rectangle '{}'",
                parts[1], s
            ))
        })?;

        Ok(Rectangle { width, height })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tree {
    space: Rectangle,
    /// The required number of gifts of each shape, identified by index.
    num_gifts: Vec<usize>,
}

impl Tree {
    fn can_fit(&self, gifts: &[Gift]) -> bool {
        let minimum_area: usize = gifts
            .iter()
            .enumerate()
            .map(|(i, g)| self.num_gifts[i] * g.area)
            .sum();
        let area = self.space.area();
        if minimum_area > area {
            return false;
        }

        let mut placements = PlacementSpace::new(self.space.width, self.space.height);
        let mut gifts_to_place = self.num_gifts.clone();

        while gifts_to_place.iter().sum::<usize>() > 0 {
            if let Some((gift_idx, _placed_gift)) =
                placements.place_next_gift(&gifts_to_place, gifts)
            {
                gifts_to_place[gift_idx] -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

impl FromStr for Tree {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::ParseError(format!(
                "Invalid tree format '{}': expected 'widthxheight: gift1 gift2 ...'",
                s
            )));
        }

        let space = parts[0].trim().parse::<Rectangle>()?;

        let gifts = parts[1]
            .split_whitespace()
            .map(|num_str| {
                num_str.parse::<usize>().map_err(|_| {
                    Error::ParseError(format!("Invalid gift count '{}' in tree '{}'", num_str, s))
                })
            })
            .collect::<Result<Vec<usize>, Error>>()?;

        Ok(Tree {
            space,
            num_gifts: gifts,
        })
    }
}

/// Clockwise rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

struct PlacedGift<'a> {
    location: Point2<i32>,
    view: &'a ShapeView,
}

impl<'a> PlacedGift<'a> {
    fn new(location: Point2<i32>, view: &'a ShapeView) -> Self {
        Self { location, view }
    }

    fn view(&self) -> &ShapeView {
        &self.view
    }

    fn occupied_world_pixels(&self) -> impl Iterator<Item = Point2<i32>> + '_ {
        let shape = self.view();
        let location = self.location;

        shape
            .occupied_pixels()
            .map(move |(row, col)| location + Vector2::new(col as i32, row as i32))
    }

    fn bounding_box(&self) -> (Point2<i32>, Point2<i32>) {
        let shape = self.view();
        let top_left = self.location;
        let bottom_right = top_left + Vector2::new(shape.width as i32, shape.height as i32);
        (top_left, bottom_right)
    }

    fn contains_point(&self, world_point: Point2<i32>) -> bool {
        let shape = self.view();
        let local = world_point - self.location.coords;

        if local.x < 0 || local.y < 0 {
            return false;
        }

        let col = local.x as usize;
        let row = local.y as usize;

        if row >= shape.height || col >= shape.width {
            return false;
        }

        shape.get_pixel(row, col) == Pixel::Gift
    }
}

struct PlacementSpace {
    width: usize,
    height: usize,
    /// Flat grid tracking which cells are occupied.
    occupied: Vec<bool>,
}

impl PlacementSpace {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            occupied: vec![false; width * height],
        }
    }

    fn is_occupied(&self, point: Point2<i32>) -> bool {
        if point.x < 0 || point.y < 0 {
            return true; // Out of bounds = occupied
        }

        let x = point.x as usize;
        let y = point.y as usize;

        if x >= self.width || y >= self.height {
            return true;
        }

        self.occupied[y * self.width + x]
    }

    fn can_place(&self, gift: &PlacedGift) -> bool {
        // Check bounds
        let (top_left, bottom_right) = gift.bounding_box();
        if top_left.x < 0 || top_left.y < 0 {
            return false;
        }
        if bottom_right.x > self.width as i32 || bottom_right.y > self.height as i32 {
            return false;
        }

        // Check pixel-level collision
        gift.occupied_world_pixels().all(|p| !self.is_occupied(p))
    }

    fn place(&mut self, gift: &PlacedGift) {
        for point in gift.occupied_world_pixels() {
            let x = point.x as usize;
            let y = point.y as usize;
            self.occupied[y * self.width + x] = true;
        }
    }

    fn place_next_gift<'g>(
        &mut self,
        gifts_to_place: &[usize],
        gifts: &'g [Gift],
    ) -> Option<(usize, PlacedGift<'g>)> {
        let mut fewest_blocked_points = usize::MAX;
        let mut best_gift = None;

        for x in 0..self.height {
            if fewest_blocked_points == 0 {
                break;
            }
            let mut is_all_unoccupied = true;
            for y in 0..self.width {
                if fewest_blocked_points == 0 {
                    break;
                }
                let at = point!(x as i32, y as i32);
                if self.is_occupied(at) {
                    is_all_unoccupied = false;
                    continue;
                }
                for (gift_idx, _) in gifts_to_place
                    .iter()
                    .enumerate()
                    .filter(|(_idx, num)| **num != 0)
                {
                    let gift = &gifts[gift_idx];
                    let other_available_gifts: Vec<_> = gifts_to_place
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, num)| {
                            if (idx != gift_idx && *num != 0) || (idx == gift_idx && *num > 1) {
                                Some(&gifts[idx])
                            } else {
                                None
                            }
                        })
                        .collect();
                    let valid_placements = self.find_valid_placements_at(at, gift);
                    for placed_gift in valid_placements {
                        let num_blocked_points =
                            self.count_blocked_points(&placed_gift, &other_available_gifts);
                        if num_blocked_points < fewest_blocked_points {
                            fewest_blocked_points = num_blocked_points;
                            best_gift = Some((gift_idx, placed_gift));
                        }
                    }
                }
            }
            // Once we have tackled a fully unoccupied row, we can exit.
            if is_all_unoccupied {
                break;
            }
        }

        // Place the best gift if there is one.
        if let Some((_, gift)) = &best_gift {
            self.place(gift);
        }

        best_gift
    }

    /// Finds a valid placement, if possible, for the given gift such that it has a non-empty pixel
    /// on the given point.
    fn find_valid_placements_at<'g>(
        &self,
        at: Point2<i32>,
        gift: &'g Gift,
    ) -> impl Iterator<Item = PlacedGift<'g>> {
        let mut placed_gifts = Vec::with_capacity(4);
        for (i, view) in gift.views.iter().enumerate() {
            let top_left = view.occupied_pixels().next().unwrap();
            let required_offset = at - point![top_left.0 as i32, top_left.1 as i32];
            let placed_gift = PlacedGift::new(Point2::from(required_offset), view);
            if self.can_place(&placed_gift) {
                placed_gifts.push(placed_gift);
            }
        }
        placed_gifts.into_iter()
    }

    /// Counts the number of points that become inaccessible after placing this point.
    fn count_blocked_points(&self, gift: &PlacedGift, available_gifts: &[&Gift]) -> usize {
        let mut count_blocked = 0;

        let min_gift_size = available_gifts.iter().map(|g| g.area).min().unwrap_or(0);

        let mut checked_points: HashSet<Point2<i32>> = HashSet::new();
        for point in gift.occupied_world_pixels() {
            for x_shift in -1..=1 {
                for y_shift in -1..=1 {
                    let shift = point + Vector2::new(x_shift, y_shift);
                    if self.is_occupied(shift)
                        || gift.contains_point(shift)
                        || checked_points.contains(&shift)
                    {
                        continue;
                    }
                    // Just consider it a block if the gap is smaller than any gift (for simplicity).
                    let gap_points: HashSet<_> = iterate_contiguous(shift, |point| {
                        self.is_occupied(point) || gift.contains_point(point)
                    })
                    .take(min_gift_size) // Take only up-to the min gift-size to early exit.
                    .collect();
                    let old_checked_points_size = checked_points.len();
                    let gap_size = gap_points.len();
                    checked_points.extend(gap_points);
                    if checked_points.len() - old_checked_points_size < gap_size {
                        // This overlaps with a seen gap, no need to re-add it.
                        continue;
                    }
                    if gap_size < min_gift_size {
                        count_blocked += gap_size;
                    }
                }
            }
        }
        count_blocked
    }
}

struct LocationIter<StopFn>
where
    StopFn: Fn(Point2<i32>) -> bool,
{
    seen: HashSet<Point2<i32>>,
    to_visit: VecDeque<Point2<i32>>,
    stop_fn: StopFn,
}

impl<StopFn> Iterator for LocationIter<StopFn>
where
    StopFn: Fn(Point2<i32>) -> bool,
{
    type Item = Point2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.to_visit.pop_front() else {
            return None;
        };

        for x in -1..=1 {
            for y in -1..=1 {
                let point = point![x, y];
                if self.seen.contains(&point) {
                    continue;
                }
                if point.x < 0 || point.y < 0 {
                    continue;
                }
                if (self.stop_fn)(point) {
                    continue;
                }
                self.seen.insert(point);
                self.to_visit.push_back(point);
            }
        }

        Some(next)
    }
}

/// Iterate contiguous locations from a source matching some condition.
fn iterate_contiguous<StopFn: Fn(Point2<i32>) -> bool>(
    source: Point2<i32>,
    stop_fn: StopFn,
) -> LocationIter<StopFn> {
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    seen.insert(source);
    to_visit.push_back(source);
    LocationIter {
        seen,
        to_visit,
        stop_fn,
    }
}

fn parse_gifts_and_trees(input: &str) -> Result<(Vec<Gift>, Vec<Tree>), Error> {
    let mut gifts = Vec::new();
    let mut trees = Vec::new();

    let empty_line_regex = Regex::new("\n\n").unwrap();
    let gift_id_regex = Regex::new(r"^\d+:\n").unwrap();
    for part in empty_line_regex.split(input) {
        if gift_id_regex.is_match(part) {
            let gift_part = gift_id_regex.replace(part, "");
            gifts.push(gift_part.parse()?);
        } else {
            for tree_part in part.lines() {
                if tree_part.is_empty() {
                    continue;
                }

                trees.push(tree_part.parse()?);
            }
        }
    }

    Ok((gifts, trees))
}

pub fn part1(input: &str) -> AnyResult<usize> {
    let (gifts, trees) = parse_gifts_and_trees(input)?;

    Ok(trees
        .iter()
        .enumerate()
        .filter(|(i, t)| {
            let can_fit = t.can_fit(&gifts);
            if !can_fit {
                log::info!("Failed to fit tree {i}");
            }
            can_fit
        })
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gift_parse() {
        let result: Result<Gift, Error> = "###
        ##.
        .##"
        .parse();

        assert!(result.is_ok(), "{:?}", &result);

        let gift = result.unwrap();
        assert_eq!(gift.area, 7);

        // Verify Rot0 shape (original)
        let rot0 = &gift.views[0];
        assert_eq!(rot0.width, 3);
        assert_eq!(rot0.height, 3);
        assert_eq!(rot0.get_pixel(0, 0), Pixel::Gift);
        assert_eq!(rot0.get_pixel(0, 1), Pixel::Gift);
        assert_eq!(rot0.get_pixel(0, 2), Pixel::Gift);
        assert_eq!(rot0.get_pixel(1, 0), Pixel::Gift);
        assert_eq!(rot0.get_pixel(1, 1), Pixel::Gift);
        assert_eq!(rot0.get_pixel(1, 2), Pixel::Empty);
        assert_eq!(rot0.get_pixel(2, 0), Pixel::Empty);
        assert_eq!(rot0.get_pixel(2, 1), Pixel::Gift);
        assert_eq!(rot0.get_pixel(2, 2), Pixel::Gift);
    }

    #[test]
    fn test_gift_parse_mismatched_length() {
        let result: Result<Gift, Error> = "###
        ##.#
        .##"
        .parse();

        assert!(result.is_err(), "{:?}", &result);
    }

    #[test]
    fn test_gift_parse_empty_lines() {
        let result: Result<Gift, Error> = "
        ##.

        #..
        ..."
        .parse();

        assert!(result.is_ok(), "{:?}", &result);
        let gift = result.unwrap();
        assert_eq!(gift.area, 3);

        // Verify Rot0 shape
        let rot0 = &gift.views[0];
        assert_eq!(rot0.width, 3);
        assert_eq!(rot0.height, 3);
        assert_eq!(rot0.get_pixel(0, 0), Pixel::Gift);
        assert_eq!(rot0.get_pixel(0, 1), Pixel::Gift);
        assert_eq!(rot0.get_pixel(1, 0), Pixel::Gift);
    }

    #[test]
    fn test_rectangle_parse() {
        let result: Result<Rectangle, Error> = "12x5".parse();
        assert!(result.is_ok(), "{:?}", &result);

        let rect = result.unwrap();
        assert_eq!(rect.width, 12);
        assert_eq!(rect.height, 5);
        assert_eq!(rect.area(), 60);
    }

    #[test]
    fn test_rectangle_parse_invalid() {
        // Missing 'x'
        let result: Result<Rectangle, Error> = "4-4".parse();
        assert!(result.is_err(), "{:?}", &result);

        // Non-numeric width
        let result: Result<Rectangle, Error> = "axb".parse();
        assert!(result.is_err(), "{:?}", &result);

        // Too many parts
        let result: Result<Rectangle, Error> = "4x4x4".parse();
        assert!(result.is_err(), "{:?}", &result);

        // Empty string
        let result: Result<Rectangle, Error> = "".parse();
        assert!(result.is_err(), "{:?}", &result);
    }

    #[test]
    fn test_tree_parse() {
        let result: Result<Tree, Error> = "4x5: 0 0 0 10 2 0".parse();
        assert!(result.is_ok(), "{:?}", &result);

        let tree = result.unwrap();
        assert_eq!(tree.space.width, 4);
        assert_eq!(tree.space.height, 5);
        assert_eq!(tree.num_gifts, vec![0, 0, 0, 10, 2, 0]);
    }

    #[test]
    fn test_tree_parse_invalid() {
        // Missing colon
        let result: Result<Tree, Error> = "4x4 0 0 0 0 2 0".parse();
        assert!(result.is_err(), "{:?}", &result);

        // Invalid rectangle format
        let result: Result<Tree, Error> = "4-4: 0 0 0".parse();
        assert!(result.is_err(), "{:?}", &result);

        // Invalid gift number
        let result: Result<Tree, Error> = "4x4: 0 0 a 0".parse();
        assert!(result.is_err(), "{:?}", &result);

        // Empty gift list (valid - should parse to empty Vec)
        let result: Result<Tree, Error> = "4x4: ".parse();
        assert!(result.is_ok(), "{:?}", &result);
        let tree = result.unwrap();
        assert_eq!(tree.num_gifts.len(), 0);
    }

    #[test]
    fn test_parse_gifts_and_trees() {
        let result = parse_gifts_and_trees(
            "0:
###
##.
##.

1:
###
##.
.##

4x4: 0 1
4x6: 1 0",
        );

        assert!(result.is_ok(), "{:?}", result);
        let (gifts, trees) = result.unwrap();
        assert_eq!(gifts.len(), 2);
        assert_eq!(trees.len(), 2);
    }

    #[test]
    fn test_rotation_90() {
        // Test a simple L-shape rotated 90 degrees
        let gift: Gift = "##
.#"
        .parse()
        .unwrap();

        let rot90 = &gift.views[1];
        assert_eq!(rot90.width, 2); // Dimensions swapped
        assert_eq!(rot90.height, 2);

        // Original:    After 90° CW:
        // ##           .#
        // .#           ##
        assert_eq!(rot90.get_pixel(0, 0), Pixel::Empty);
        assert_eq!(rot90.get_pixel(0, 1), Pixel::Gift);
        assert_eq!(rot90.get_pixel(1, 0), Pixel::Gift);
        assert_eq!(rot90.get_pixel(1, 1), Pixel::Gift);
    }

    #[test]
    fn test_rotation_180() {
        let gift: Gift = "##
.#"
        .parse()
        .unwrap();

        let rot180 = &gift.views[2];
        // Original:    After 180°:
        // ##           #.
        // .#           ##
        assert_eq!(rot180.get_pixel(0, 0), Pixel::Gift);
        assert_eq!(rot180.get_pixel(0, 1), Pixel::Empty);
        assert_eq!(rot180.get_pixel(1, 0), Pixel::Gift);
        assert_eq!(rot180.get_pixel(1, 1), Pixel::Gift);
    }

    #[test]
    fn test_placement_space_basic() {
        let mut space = PlacementSpace::new(5, 5);

        // Empty space should allow placement anywhere in bounds
        assert!(!space.is_occupied(Point2::new(0, 0)));
        assert!(!space.is_occupied(Point2::new(4, 4)));

        // Out of bounds should be considered occupied
        assert!(space.is_occupied(Point2::new(-1, 0)));
        assert!(space.is_occupied(Point2::new(0, -1)));
        assert!(space.is_occupied(Point2::new(5, 0)));
        assert!(space.is_occupied(Point2::new(0, 5)));

        // Mark a cell as occupied
        let gift: Gift = "#".parse().unwrap();
        let placed = PlacedGift::new(Point2::new(2, 2), &gift.views[0]);
        space.place(&placed);

        assert!(space.is_occupied(Point2::new(2, 2)));
        assert!(!space.is_occupied(Point2::new(2, 3)));
    }

    #[test]
    fn test_can_place_collision() {
        let mut space = PlacementSpace::new(4, 4);

        let gift: Gift = "##
##"
        .parse()
        .unwrap();

        // Place first gift at (0, 0)
        let placed1 = PlacedGift::new(Point2::new(0, 0), &gift.views[0]);
        assert!(space.can_place(&placed1));
        space.place(&placed1);

        // Try to place overlapping gift - should fail
        let placed2 = PlacedGift::new(Point2::new(1, 1), &gift.views[0]);
        assert!(!space.can_place(&placed2));

        // Place adjacent gift - should succeed
        let placed3 = PlacedGift::new(Point2::new(2, 0), &gift.views[0]);
        assert!(space.can_place(&placed3));
    }

    #[test]
    fn test_can_place_out_of_bounds() {
        let space = PlacementSpace::new(3, 3);
        let gift: Gift = "##
##"
        .parse()
        .unwrap();

        // Gift would extend beyond right edge
        let placed = PlacedGift::new(Point2::new(2, 0), &gift.views[0]);
        assert!(!space.can_place(&placed));

        // Gift would extend beyond bottom edge
        let placed = PlacedGift::new(Point2::new(0, 2), &gift.views[0]);
        assert!(!space.can_place(&placed));

        // Negative position
        let placed = PlacedGift::new(Point2::new(-1, 0), &gift.views[0]);
        assert!(!space.can_place(&placed));
    }

    #[test]
    fn test_occupied_world_pixels() {
        let gift: Gift = ".#
##"
        .parse()
        .unwrap();

        let placed = PlacedGift::new(Point2::new(1, 1), &gift.views[0]);
        let pixels: Vec<_> = placed.occupied_world_pixels().collect();

        // Original shape has 3 Gift pixels at (0,1), (1,0), (1,1)
        // Offset by location (1, 1): (1,2), (2,1), (2,2)
        assert_eq!(pixels.len(), 3);
        assert!(pixels.contains(&Point2::new(2, 1))); // col=1, row=0 -> (1+1, 1+0)
        assert!(pixels.contains(&Point2::new(1, 2))); // col=0, row=1 -> (1+0, 1+1)
        assert!(pixels.contains(&Point2::new(2, 2))); // col=1, row=1 -> (1+1, 1+1)
    }

    #[test]
    fn test_part1() {
        let result = part1(
            "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2",
        );

        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), 2);
    }
}
