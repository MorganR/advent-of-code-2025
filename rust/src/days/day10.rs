use std::{
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Button {
    status_bits: usize,
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
            return Ok(Self { status_bits: 0 });
        }

        let mut status_bits = 0;
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
        }

        Ok(Self { status_bits })
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        let mut first = true;
        for i in 0..usize::BITS as usize {
            if (self.status_bits >> i) & 1 == 1 {
                if !first {
                    f.write_char(',')?;
                }
                write!(f, "{}", i)?;
                first = false;
            }
        }
        f.write_char(')')
    }
}

/// Compute the new lights status if the given button is pushed.
fn push_button(button: Button, lights: &Lights) -> Lights {
    Lights {
        status_bits: lights.status_bits ^ button.status_bits,
        length: lights.length,
    }
}

struct ShortPathWork {
    /// The minimum next button index to check.
    next_idx: usize,
    /// The current status lights.
    status: Lights,
    /// Path of button indexes so far.
    path_so_far: HashSet<usize>,
}

#[derive(Debug, PartialEq)]
struct Machine {
    target: Lights,
    buttons: Vec<Button>,
}

impl Machine {
    /// Compute the shortest path of button presses to hit the target light status.
    ///
    /// Returns the buttons by index.
    pub fn shortest_path_to_target(&self) -> Option<HashSet<usize>> {
        let status = Lights::new(self.target.length);
        let mut seen_statuses = HashSet::new();
        seen_statuses.insert(status.clone());

        if status == self.target {
            return Some(HashSet::new());
        }

        let mut work_queue: VecDeque<ShortPathWork> = VecDeque::new();
        for idx in 0..self.buttons.len() {
            work_queue.push_back(ShortPathWork {
                next_idx: idx,
                status: status.clone(),
                path_so_far: HashSet::new(),
            });
        }

        while let Some(item) = work_queue.pop_front() {
            for b_idx in item.next_idx..self.buttons.len() {
                let next_status = push_button(self.buttons[b_idx], &item.status);
                let mut path = item.path_so_far.clone();
                path.insert(b_idx);
                if next_status == self.target {
                    return Some(path);
                } else if b_idx + 1 < self.buttons.len() {
                    work_queue.push_back(ShortPathWork {
                        next_idx: b_idx + 1,
                        status: next_status,
                        path_so_far: path,
                    });
                }
            }
        }

        None
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
        let lights: Lights = trimmed[..=lights_end].parse()?;

        // Parse buttons - everything between lights and the curly braces
        let remaining = &trimmed[lights_end + 1..];

        // Find where the data section starts (if it exists)
        let buttons_section = if let Some(data_start) = remaining.find('{') {
            &remaining[..data_start]
        } else {
            remaining
        };

        // Parse each button
        let buttons: Vec<Button> = buttons_section
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            target: lights,
            buttons,
        })
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.target.fmt(f)?;
        for button in &self.buttons {
            f.write_char(' ')?;
            button.fmt(f)?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> AnyResult<usize> {
    let mut total_button_presses = 0;
    for line in input.lines() {
        let machine: Machine = line.parse()?;

        if let Some(path) = machine.shortest_path_to_target() {
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
            }
        );
        assert_eq!(format!("{}", button), "(0,1,3)");
    }

    #[test]
    fn test_button_empty() {
        let result: Result<Button, _> = "()".parse();

        assert!(result.is_ok(), "{:?}", result);
        let button = result.unwrap();
        assert_eq!(button, Button { status_bits: 0 });
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

        let new_lights = push_button(button, &lights);

        assert_eq!(format!("{}", new_lights), "[#..##]");
    }

    #[test]
    fn test_machine_from_str() {
        let result: Result<Machine, _> = "[.###] (0,2,3) (2) (1,2,3) {19,9,30,28}".parse();

        assert!(result.is_ok(), "{:?}", result);
        let machine = result.unwrap();

        assert_eq!(machine.target, "[.###]".parse().unwrap());
        assert_eq!(machine.buttons.len(), 3);
        assert_eq!(machine.buttons[0], "(0,2,3)".parse().unwrap());
        assert_eq!(machine.buttons[1], "(2)".parse().unwrap());
        assert_eq!(machine.buttons[2], "(1,2,3)".parse().unwrap());

        assert_eq!(format!("{}", machine), "[.###] (0,2,3) (2) (1,2,3)");
    }

    #[test]
    fn test_machine_without_data() {
        let result: Result<Machine, _> = "[##..] (0) (1,2)".parse();

        assert!(result.is_ok(), "{:?}", result);
        let machine = result.unwrap();

        assert_eq!(machine.target, "[##..]".parse().unwrap());
        assert_eq!(machine.buttons.len(), 2);
        assert_eq!(format!("{}", machine), "[##..] (0) (1,2)");
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
}
