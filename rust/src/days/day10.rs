use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fmt::{Display, Write},
    str::FromStr,
};

use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;

use crate::utils::input::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Power {
    On,
    Off,
}

impl Power {
    fn to_bit(self) -> usize {
        match self {
            Power::On => 1,
            Power::Off => 0,
        }
    }

    fn from_bit(bit: usize) -> Self {
        match bit & 0b1 {
            0 => Power::Off,
            _ => Power::On,
        }
    }
}

impl TryFrom<char> for Power {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Power::On),
            '.' => Ok(Power::Off),
            _ => Err(Error::ParseError(format!(
                "Invalid power character: {}",
                value
            ))),
        }
    }
}

impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Power::On => f.write_char('#'),
            Power::Off => f.write_char('.'),
        }
    }
}

impl FromStr for Power {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError(format!(
                "Power string should have length 1, had length {}",
                s.len()
            )));
        }
        let char = s.chars().next().ok_or(Error::ParseError(format!(
            "Couldn't parse the string bytes as a character, str bytes: {:?}",
            s.bytes()
        )))?;
        char.try_into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Lights {
    status_bits: usize,
    length: usize,
}

impl Lights {
    fn new(length: usize) -> Self {
        Self {
            status_bits: 0,
            length,
        }
    }
}

impl FromStr for Lights {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return Err(Error::ParseError(format!(
                "Lights string should be wrapped in square brackets: {}",
                trimmed
            )));
        }
        let mut status: Vec<Power> = Vec::new();
        for c in trimmed[1..trimmed.len() - 1].chars() {
            match c {
                '[' => {}
                ']' => {
                    break;
                }
                _ => {
                    status.push(c.try_into()?);
                }
            }
        }

        let mut status_bits = 0;
        for (i, light) in status.iter().enumerate() {
            status_bits |= light.to_bit() << i;
        }

        Ok(Self {
            status_bits,
            length: status.len(),
        })
    }
}

impl Display for Lights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for i in 0..self.length {
            let bit = self.status_bits >> i;
            let power = Power::from_bit(bit);
            power.fmt(f)?;
        }
        f.write_char(']')
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Button {
    status_bits: usize,
    indices: Vec<usize>,
}

impl Button {
    fn get_indices(&self) -> &Vec<usize> {
        &self.indices
    }

    fn width(&self) -> usize {
        self.indices.len()
    }
}

impl FromStr for Button {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
            return Err(Error::ParseError(format!(
                "Button string should be wrapped in parentheses: {}",
                s
            )));
        }

        let inner = &trimmed[1..trimmed.len() - 1];
        if inner.is_empty() {
            return Ok(Self {
                status_bits: 0,
                indices: Vec::new(),
            });
        }

        let mut status_bits = 0;
        let mut indices = Vec::new();
        for part in inner.split(',') {
            let index: usize = part
                .trim()
                .parse()
                .map_err(|_| Error::ParseError(format!("Invalid button index: {}", part)))?;

            if index >= usize::BITS as usize {
                return Err(Error::ParseError(format!(
                    "Button index {} is too large (max: {})",
                    index,
                    usize::BITS - 1
                )));
            }

            status_bits |= 1 << index;
            indices.push(index);
        }

        Ok(Self {
            status_bits,
            indices,
        })
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        for (i, index) in self.indices.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            write!(f, "{}", index)?;
        }
        f.write_char(')')
    }
}

/// Compute the new lights status if the given button is pushed.
fn push_button_for_lights(button: &Button, lights: &Lights) -> Lights {
    Lights {
        status_bits: lights.status_bits ^ button.status_bits,
        length: lights.length,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Joltage {
    values: Vec<i64>,
}

impl Joltage {
    fn new(length: usize) -> Self {
        Self {
            values: vec![0; length],
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn push(&mut self, button: &Button, n: i64) {
        for &idx in button.get_indices() {
            self.values[idx] += n;
        }
    }

    fn max_num_pushes(&self, button: &Button, target: &Joltage) -> i64 {
        let mut max_pushes = i64::MAX;
        for &idx in button.get_indices() {
            let allowed_pushes = target.values[idx] - self.values[idx];
            max_pushes = max_pushes.min(allowed_pushes);
        }
        max_pushes
    }
}

impl FromStr for Joltage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err(Error::ParseError(format!(
                "Joltage string should be wrapped in curly braces: {}",
                s
            )));
        }

        let inner = &trimmed[1..trimmed.len() - 1];
        if inner.is_empty() {
            return Ok(Self { values: Vec::new() });
        }

        let values: Vec<i64> = inner
            .split(',')
            .map(|s| {
                s.trim()
                    .parse()
                    .map_err(|_| Error::ParseError(format!("Invalid joltage value: {}", s)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { values })
    }
}

impl Display for Joltage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('{')?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
                f.write_char(',')?;
            }
            write!(f, "{}", value)?;
        }
        f.write_char('}')
    }
}

struct ShortPathLightsWorkItem {
    /// The minimum next button index to check.
    next_idx: usize,
    /// The current status lights.
    status: Lights,
    /// Path of button indexes so far.
    path_so_far: HashSet<usize>,
}

#[derive(Clone, Debug)]
struct ButtonPresses {
    presses_by_idx: Vec<i64>,
}

impl ButtonPresses {
    fn new(length: usize) -> Self {
        Self {
            presses_by_idx: vec![0; length],
        }
    }

    fn press(&mut self, idx: usize, n: i64) {
        self.presses_by_idx[idx] += n;
    }

    fn total_presses(&self) -> i64 {
        self.presses_by_idx.iter().sum()
    }
}

#[derive(Clone)]
struct JoltageWorkItem {
    /// The current joltage.
    joltage: Joltage,
    /// The minimum next button index to check.
    next_idx: usize,
    /// Path of button presses so far.
    path_so_far: ButtonPresses,
}

impl JoltageWorkItem {
    fn min_possible_presses(&self) -> i64 {
        let remaining_presses = *self.joltage.values.iter().max().unwrap();
        let presses_so_far = self.path_so_far.total_presses();
        remaining_presses + presses_so_far
    }
}

struct JoltagePathFinder<'m> {
    target: &'m Joltage,
    buttons: Vec<Button>,
    joltage: Joltage,
    orig_button_idx: Vec<usize>,
}

impl<'m> JoltagePathFinder<'m> {
    fn new(target: &'m Joltage, buttons: &'m [Button]) -> Self {
        let joltage = Joltage::new(target.len());

        let mut button_idx_by_joltage_idx: Vec<_> = (0..joltage.values.len())
            .map(|_| Vec::<usize>::new())
            .collect();
        for (idx, button) in buttons.iter().enumerate() {
            for &j_idx in button.get_indices() {
                let button_idxs = button_idx_by_joltage_idx.get_mut(j_idx).unwrap();
                button_idxs.push(idx);
            }
        }

        let mut sorted_buttons: Vec<Button> = buttons.iter().map(|b| b.clone()).collect();
        sorted_buttons.sort_unstable_by(|b1, b2| {
            // Sort in descending width order.
            let width_comp = b2.width().cmp(&b1.width());
            if width_comp != Ordering::Equal {
                return width_comp;
            }

            // Secondary sort in ascending max push order.
            let b1_max_push = joltage.max_num_pushes(b1, target);
            let b2_max_push = joltage.max_num_pushes(b2, target);
            b1_max_push.cmp(&b2_max_push)
        });
        let orig_button_idx = sorted_buttons
            .iter()
            .map(|b| {
                buttons
                    .iter()
                    .enumerate()
                    .find_map(|(i, orig_b)| if orig_b == b { Some(i) } else { None })
                    .unwrap()
            })
            .collect();

        Self {
            target,
            buttons: sorted_buttons,
            joltage,
            orig_button_idx,
        }
    }

    fn shortest_path_to_target(self) -> Option<ButtonPresses> {
        let mut work_queue: VecDeque<JoltageWorkItem> = VecDeque::new();
        for idx in 0..self.buttons.len() {
            work_queue.push_back(JoltageWorkItem {
                next_idx: idx,
                joltage: self.joltage.clone(),
                path_so_far: ButtonPresses::new(self.buttons.len()),
            });
        }

        let mut best_so_far: Option<ButtonPresses> = None;
        while let Some(item) = work_queue.pop_front() {
            if item.next_idx >= self.buttons.len() {
                continue;
            }

            if let Some(best_path) = &best_so_far
                && best_path.total_presses() <= item.min_possible_presses()
            {
                continue;
            }

            for b_idx in item.next_idx..self.buttons.len() {
                let mut next_item = item.clone();
                next_item.next_idx = b_idx + 1;
                let button = &self.buttons[b_idx];
                let num_pushes = item.joltage.max_num_pushes(button, self.target);
                if num_pushes == 0 {
                    work_queue.push_back(next_item);
                    continue;
                }

                next_item.joltage.push(button, num_pushes);
                next_item.path_so_far.press(b_idx, num_pushes);
                if next_item.joltage == *self.target {
                    best_so_far = Some(next_item.path_so_far);
                    break;
                } else {
                    work_queue.push_back(next_item);
                }
            }
        }

        best_so_far.map(|p| self.map_to_original_idx(p))
    }

    /// Map [ButtonPresses] back to the original button index.
    fn map_to_original_idx(&self, presses: ButtonPresses) -> ButtonPresses {
        if presses.total_presses() == 0 {
            return presses;
        }

        let mut idx_presses: Vec<_> = presses.presses_by_idx.iter().enumerate().collect();
        idx_presses.sort_unstable_by(|(i1, _), (i2, _)| {
            let orig_1 = self.orig_button_idx[*i1];
            let orig_2 = self.orig_button_idx[*i2];
            orig_1.cmp(&orig_2)
        });

        ButtonPresses {
            presses_by_idx: idx_presses.iter().map(|(_, p)| **p).collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Machine {
    target_lights: Lights,
    buttons: Vec<Button>,
    target_joltage: Joltage,
}

impl Machine {
    /// Compute the shortest path of button presses to hit the target light status.
    ///
    /// Returns the buttons by index.
    pub fn shortest_path_to_target_lights(&self) -> Option<HashSet<usize>> {
        let status = Lights::new(self.target_lights.length);
        let mut seen_statuses = HashSet::new();
        seen_statuses.insert(status.clone());

        if status == self.target_lights {
            return Some(HashSet::new());
        }

        let mut work_queue: VecDeque<ShortPathLightsWorkItem> = VecDeque::new();
        for idx in 0..self.buttons.len() {
            work_queue.push_back(ShortPathLightsWorkItem {
                next_idx: idx,
                status: status.clone(),
                path_so_far: HashSet::new(),
            });
        }

        while let Some(item) = work_queue.pop_front() {
            for b_idx in item.next_idx..self.buttons.len() {
                let next_status = push_button_for_lights(&self.buttons[b_idx], &item.status);
                let mut path = item.path_so_far.clone();
                path.insert(b_idx);
                if next_status == self.target_lights {
                    return Some(path);
                } else if b_idx + 1 < self.buttons.len() {
                    work_queue.push_back(ShortPathLightsWorkItem {
                        next_idx: b_idx + 1,
                        status: next_status,
                        path_so_far: path,
                    });
                }
            }
        }

        None
    }

    pub fn shortest_path_to_target_joltage(&self) -> Option<ButtonPresses> {
        let path_finder = JoltagePathFinder::new(&self.target_joltage, &self.buttons);

        path_finder.shortest_path_to_target()
    }
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        // Find the lights section (enclosed in [])
        let lights_end = trimmed
            .find(']')
            .ok_or_else(|| Error::ParseError("Missing closing bracket for lights".to_string()))?;
        let target_lights: Lights = trimmed[..=lights_end].parse()?;

        // Parse buttons - everything between lights and the curly braces
        let remaining = &trimmed[lights_end + 1..];

        // Find where the data section starts
        let (buttons_section, joltage_section) = if let Some(data_start) = remaining.find('{') {
            (&remaining[..data_start], &remaining[data_start..])
        } else {
            (remaining, "{}")
        };

        // Parse each button
        let buttons: Vec<Button> = buttons_section
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        // Parse joltage
        let target_joltage: Joltage = joltage_section.parse()?;

        Ok(Self {
            target_lights,
            buttons,
            target_joltage,
        })
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.target_lights.fmt(f)?;
        for button in &self.buttons {
            f.write_char(' ')?;
            button.fmt(f)?;
        }
        f.write_char(' ')?;
        self.target_joltage.fmt(f)
    }
}

pub fn part1(input: &str) -> AnyResult<usize> {
    let mut total_button_presses = 0;
    for line in input.lines() {
        let machine: Machine = line.parse()?;

        if let Some(path) = machine.shortest_path_to_target_lights() {
            log::debug!("Found shortest path: {:?}", path);
            total_button_presses += path.len();
        } else {
            return Err(AnyError::new(Error::LogicError(format!(
                "Failed to find path for machine {}",
                machine
            ))));
        }
    }
    Ok(total_button_presses)
}

pub fn part2(input: &str) -> AnyResult<i64> {
    let mut total_button_presses = 0;
    for line in input.lines() {
        let machine: Machine = line.parse()?;
        if let Some(path) = machine.shortest_path_to_target_joltage() {
            log::debug!("Found shortest path: {:?}", path);
            total_button_presses += path.total_presses();
        } else {
            return Err(AnyError::new(Error::LogicError(format!(
                "Failed to find path for machine {}",
                machine
            ))));
        }
    }
    Ok(total_button_presses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lights_from_str() {
        let result: Result<Lights, _> = "[#.##.]".parse();

        assert!(result.is_ok(), "{:?}", result);
        let lights = result.unwrap();
        assert_eq!(
            lights,
            Lights {
                status_bits: 0b1101,
                length: 5,
            }
        );
        assert_eq!(format!("{}", lights), "[#.##.]");
    }

    #[test]
    fn test_lights_from_str_empty() {
        let result: Result<Lights, _> = "[]".parse();

        assert!(result.is_ok(), "{:?}", result);
        let lights = result.unwrap();
        assert_eq!(
            lights,
            Lights {
                status_bits: 0,
                length: 0,
            }
        );
        assert_eq!(format!("{}", lights), "[]");
    }

    #[test]
    fn test_button_from_str() {
        let result: Result<Button, _> = "(0,1,3)".parse();

        assert!(result.is_ok(), "{:?}", result);
        let button = result.unwrap();
        assert_eq!(
            button,
            Button {
                status_bits: 0b1011,
                indices: vec![0, 1, 3],
            }
        );
        assert_eq!(format!("{}", button), "(0,1,3)");
    }

    #[test]
    fn test_button_empty() {
        let result: Result<Button, _> = "()".parse();

        assert!(result.is_ok(), "{:?}", result);
        let button = result.unwrap();
        assert_eq!(
            button,
            Button {
                status_bits: 0,
                indices: vec![],
            }
        );
        assert_eq!(format!("{}", button), "()");
    }

    #[test]
    fn test_button_single() {
        let result: Result<Button, _> = "(5)".parse();

        assert!(result.is_ok(), "{:?}", result);
        let button = result.unwrap();
        assert_eq!(
            button,
            Button {
                status_bits: 0b100000,
                indices: vec![5],
            }
        );
        assert_eq!(format!("{}", button), "(5)");
    }

    #[test]
    fn test_button_invalid_index() {
        let result: Result<Button, _> = format!("({})", usize::BITS).parse();

        assert!(result.is_err());
        assert!(format!("{:?}", result).contains("too large"));
    }

    #[test]
    fn test_push_button() {
        let lights: Lights = "[##...]".parse().unwrap();
        let button: Button = "(1,3,4)".parse().unwrap();

        let new_lights = push_button_for_lights(&button, &lights);

        assert_eq!(format!("{}", new_lights), "[#..##]");
    }

    #[test]
    fn test_joltage_from_str() {
        let result: Result<Joltage, _> = "{19,9,30,28}".parse();

        assert!(result.is_ok(), "{:?}", result);
        let joltage = result.unwrap();
        assert_eq!(
            joltage,
            Joltage {
                values: vec![19, 9, 30, 28],
            }
        );
        assert_eq!(format!("{}", joltage), "{19,9,30,28}");
    }

    #[test]
    fn test_joltage_empty() {
        let result: Result<Joltage, _> = "{}".parse();

        assert!(result.is_ok(), "{:?}", result);
        let joltage = result.unwrap();
        assert_eq!(joltage, Joltage { values: vec![] });
        assert_eq!(format!("{}", joltage), "{}");
    }

    #[test]
    fn test_joltage_single() {
        let result: Result<Joltage, _> = "{42}".parse();

        assert!(result.is_ok(), "{:?}", result);
        let joltage = result.unwrap();
        assert_eq!(joltage, Joltage { values: vec![42] });
        assert_eq!(format!("{}", joltage), "{42}");
    }

    #[test]
    fn test_machine_from_str() {
        let result: Result<Machine, _> = "[.###] (0,2,3) (2) (1,2,3) {19,9,30,28}".parse();

        assert!(result.is_ok(), "{:?}", result);
        let machine = result.unwrap();

        assert_eq!(machine.target_lights, "[.###]".parse().unwrap());
        assert_eq!(machine.buttons.len(), 3);
        assert_eq!(machine.buttons[0], "(0,2,3)".parse().unwrap());
        assert_eq!(machine.buttons[1], "(2)".parse().unwrap());
        assert_eq!(machine.buttons[2], "(1,2,3)".parse().unwrap());
        assert_eq!(machine.target_joltage, "{19,9,30,28}".parse().unwrap());

        assert_eq!(
            format!("{}", machine),
            "[.###] (0,2,3) (2) (1,2,3) {19,9,30,28}"
        );
    }

    #[test]
    fn test_machine_shortest_joltage() {
        let machine: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();

        let maybe_path = machine.shortest_path_to_target_joltage();

        assert!(maybe_path.is_some());
        let path = maybe_path.unwrap();
        assert_eq!(path.total_presses(), 10, "{:?}", path);
    }

    #[test]
    fn test_machine_shortest_joltage_tough() {
        //                                   0     1           2         3   4       5                 6         7     8           9                 0  1  2  3  4  5  6  7  8  9
        let machine: Machine = "[.#......#.] (2,9) (3,5,6,7,8) (0,7,8,9) (4) (0,2,3) (2,3,4,5,6,7,8,9) (1,2,3,7) (1,8) (0,2,5,6,9) (0,1,2,3,5,6,7) {59,48,81,71,11,42,42,70,42,42}"
            .parse()  
            .unwrap();

        let maybe_path = machine.shortest_path_to_target_joltage();

        assert!(maybe_path.is_some());
    }

    #[test]
    fn test_machine_without_data() {
        let result: Result<Machine, _> = "[##..] (0) (1,2)".parse();

        assert!(result.is_ok(), "{:?}", result);
        let machine = result.unwrap();

        assert_eq!(machine.target_lights, "[##..]".parse().unwrap());
        assert_eq!(machine.buttons.len(), 2);
        assert_eq!(machine.target_joltage, "{}".parse().unwrap());
        assert_eq!(format!("{}", machine), "[##..] (0) (1,2) {}");
    }

    #[test]
    fn test_part1() {
        let result = part1(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );

        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        let result = part2(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );

        assert!(result.is_ok(), "{:?}", result);
        assert_eq!(result.unwrap(), 33);
    }
}
