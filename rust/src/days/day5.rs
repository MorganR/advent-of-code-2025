use crate::utils::input::Error;
use std::cmp::{Ordering, max, min};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Range(u64, u64);

impl Range {
    fn parse(input: &str) -> Result<Range, Error> {
        let values: Vec<&str> = input.split('-').collect();
        if values.len() != 2 {
            return Err(Error::ParseError(format!("invalid range: {}", input)));
        }
        let start = values[0]
            .parse()
            .map_err(|err| Error::ParseError(format!("could not parse start: {:?}", err)))?;
        let end = values[1]
            .parse()
            .map_err(|err| Error::ParseError(format!("could not parse end: {:?}", err)))?;
        Ok(Range(start, end))
    }

    fn compare_value(&self, value: u64) -> Ordering {
        if value > self.1 {
            Ordering::Greater
        } else if value < self.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.0 > other.1 || self.1 < other.0)
    }

    fn merge(&mut self, other: &Range) {
        self.0 = min(self.0, other.0);
        self.1 = max(self.1, other.1);
    }
}

fn is_in_ranges(value: u64, ordered_ranges: &Vec<Range>) -> bool {
    let find_result = ordered_ranges.binary_search_by(|range| range.compare_value(value).reverse());
    find_result.is_ok()
}

fn parse_ordered_ranges(input: &str) -> Result<Vec<Range>, Error> {
    let ranges_result: Result<Vec<Range>, Error> = input
        .lines()
        .into_iter()
        .map(|line| Range::parse(line))
        .collect();
    let mut ranges = ranges_result?;

    let mut did_merge = true;
    while did_merge {
        did_merge = false;
        let mut i = 0;
        while i < ranges.len() {
            let mut j = i + 1;
            while j < ranges.len() {
                if ranges[i].overlaps(&ranges[j]) {
                    let old_range = ranges.swap_remove(j);
                    ranges[i].merge(&old_range);
                    did_merge = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    ranges.sort_unstable_by_key(|r| r.0);
    Ok(ranges)
}

fn parse_ingredients_and_ids(input: &str) -> Result<(Vec<Range>, Vec<u64>), Error> {
    let split: Vec<&str> = input.split("\n\n").collect();
    if split.len() != 2 {
        return Err(Error::ParseError(format!(
            "ingredients input must have an empty line between ranges and ids, received: {}",
            input
        )));
    }

    let ranges = parse_ordered_ranges(split[0])?;

    let id_strs = split[1];
    let ids: Result<Vec<u64>, Error> = id_strs
        .split('\n')
        .map(|id_str| {
            id_str.parse().map_err(|err| {
                Error::ParseError(format!("invalid ingredient id {}, err: {:?}", id_str, err))
            })
        })
        .collect();
    Ok((ranges, ids?))
}

pub fn count_fresh_ingredients(input: &str) -> Result<u64, Error> {
    let (ranges, ids) = parse_ingredients_and_ids(input)?;

    let mut count_fresh = 0;
    for id in ids {
        if is_in_ranges(id, &ranges) {
            count_fresh += 1;
        }
    }

    Ok(count_fresh)
}

pub fn count_all_fresh_ids(input: &str) -> Result<u64, Error> {
    let range_str = input.split("\n\n").next().map_or_else(|| Err(Error::ParseError(format!("invalid input: {}", input))), |v| Ok(v))?;
    let ranges = parse_ordered_ranges(range_str)?;

    let mut count = 0;
    for range in ranges {
        count += range.1 - range.0 + 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let range = Range(5, 8);
        assert_eq!(range.compare_value(4), Ordering::Less);
        assert_eq!(range.compare_value(5), Ordering::Equal);
        assert_eq!(range.compare_value(6), Ordering::Equal);
        assert_eq!(range.compare_value(8), Ordering::Equal);
        assert_eq!(range.compare_value(9), Ordering::Greater);
    }

    #[test]
    fn test_overlaps_sub_range() {
        assert!(Range(4, 7).overlaps(&Range(3, 8)));
    }

    #[test]
    fn test_overlaps_super_range() {
        assert!(Range(3, 8).overlaps(&Range(4, 7)));
    }

    #[test]
    fn test_overlaps_low_overlap() {
        assert!(Range(3, 8).overlaps(&Range(2, 5)));
        assert!(Range(3, 8).overlaps(&Range(2, 3)));
    }

    #[test]
    fn test_overlaps_high_overlap() {
        assert!(Range(5, 8).overlaps(&Range(6, 9)));
        assert!(Range(5, 8).overlaps(&Range(8, 9)));
    }

    #[test]
    fn test_no_overlap() {
        assert!(!Range(5, 8).overlaps(&Range(1, 4)));
        assert!(!Range(5, 8).overlaps(&Range(9, 11)));
    }

    #[test]
    fn test_merge_same() {
        let mut range = Range(1, 4);
        range.merge(&Range(1, 4));
        assert_eq!(range, Range(1, 4));
    }

    #[test]
    fn test_merge_up() {
        let mut range = Range(1, 4);
        range.merge(&Range(3, 8));
        assert_eq!(range, Range(1, 8));
    }

    #[test]
    fn test_merge_down() {
        let mut range = Range(3, 5);
        range.merge(&Range(1, 4));
        assert_eq!(range, Range(1, 5));
    }

    #[test]
    fn test_merge_sub_range() {
        let mut range = Range(2, 6);
        range.merge(&Range(3, 4));
        assert_eq!(range, Range(2, 6));
    }

    #[test]
    fn test_merge_super_range() {
        let mut range = Range(2, 6);
        range.merge(&Range(1, 7));
        assert_eq!(range, Range(1, 7));
    }

    #[test]
    fn test_is_in_ranges() {
        let ranges = vec![Range(1, 7), Range(9, 10), Range(12, 15)];

        assert!(is_in_ranges(3, &ranges));
        assert!(is_in_ranges(9, &ranges));
        assert!(!is_in_ranges(8, &ranges));
        assert!(!is_in_ranges(0, &ranges));
        assert!(!is_in_ranges(16, &ranges));
    }

    #[test]
    fn test_parse_ordered_ranges() {
        let ranges_result = parse_ordered_ranges(
            "1-1
6-9
3-7
13-15
10-11",
        );
        assert!(ranges_result.is_ok());
        assert_eq!(
            ranges_result.unwrap(),
            vec!(Range(1, 1), Range(3, 9), Range(10, 11), Range(13, 15))
        );
    }

    #[test]
    fn test_count_fresh_ingredients() {
        let count_result = count_fresh_ingredients(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert!(count_result.is_ok());
        assert_eq!(count_result.unwrap(), 3);
    }
}
