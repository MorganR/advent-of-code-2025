use std::collections::{HashMap, HashSet};

use crate::utils::input::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TachyonManifoldItem {
    BeamStart,
    Splitter,
    Empty,
}

impl TryFrom<char> for TachyonManifoldItem {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(TachyonManifoldItem::Splitter),
            'S' => Ok(TachyonManifoldItem::BeamStart),
            '.' => Ok(TachyonManifoldItem::Empty),
            _ => Err(Error::ParseError(format!(
                "unexpected manifold item {}, should be ^, S, or .",
                value
            ))),
        }
    }
}

pub fn count_beam_splits(input: &str) -> Result<u64, Error> {
    let mut beam_splits = 0;
    let mut beam_idxs = HashSet::new();
    let mut next_beam_idxs = HashSet::new();
    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            match TachyonManifoldItem::try_from(c)? {
                TachyonManifoldItem::BeamStart => {
                    next_beam_idxs.insert(idx);
                }
                TachyonManifoldItem::Empty => {
                    if beam_idxs.contains(&idx) {
                        next_beam_idxs.insert(idx);
                    }
                }
                TachyonManifoldItem::Splitter => {
                    if beam_idxs.contains(&idx) {
                        beam_splits += 1;
                        next_beam_idxs.insert(idx - 1);
                        next_beam_idxs.insert(idx + 1);
                    }
                }
            };
        }
        beam_idxs = next_beam_idxs;
        next_beam_idxs = HashSet::new();
    }
    Ok(beam_splits)
}

pub fn count_timelines(input: &str) -> Result<u64, Error> {
    let mut beam_timelines_by_idx: HashMap<usize, u64> = HashMap::new();
    let mut next_beam_timelines_by_idx = HashMap::new();
    for line in input.lines() {
        for (idx, c) in line.chars().enumerate() {
            match TachyonManifoldItem::try_from(c)? {
                TachyonManifoldItem::BeamStart => {
                    next_beam_timelines_by_idx.insert(idx, 1);
                }
                TachyonManifoldItem::Empty => {
                    if let Some(&num_timelines_above) = beam_timelines_by_idx.get(&idx) {
                        let &timelines = next_beam_timelines_by_idx.get(&idx).unwrap_or(&0);
                        next_beam_timelines_by_idx.insert(idx, timelines + num_timelines_above);
                    }
                }
                TachyonManifoldItem::Splitter => {
                    if let Some(num_timelines_above) = beam_timelines_by_idx.get(&idx) {
                        let mut split_idxs = Vec::with_capacity(2);
                        if idx > 0 {
                            split_idxs.push(idx - 1);
                        }
                        split_idxs.push(idx + 1);
                        for split_idx in split_idxs {
                            let &timelines =
                                next_beam_timelines_by_idx.get(&split_idx).unwrap_or(&0);
                            next_beam_timelines_by_idx
                                .insert(split_idx, timelines + num_timelines_above);
                        }
                    }
                }
            };
        }
        beam_timelines_by_idx = next_beam_timelines_by_idx;
        next_beam_timelines_by_idx = HashMap::new();
    }
    Ok(beam_timelines_by_idx.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_beam_splits_small() {
        let result = count_beam_splits(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert!(result.is_ok(), "Error: {:?}", result.err());
        assert_eq!(result.unwrap(), 21);
    }

    #[test]
    fn count_timelines_small() {
        let result = count_timelines(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert!(result.is_ok(), "Error: {:?}", result.err());
        assert_eq!(result.unwrap(), 40);
    }
}
