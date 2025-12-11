use std::str::FromStr;

use crate::utils::input::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError(format!(
                "Operator string must have length 1, received: {}",
                s
            )));
        }
        let c = s.chars().next().unwrap();

        c.try_into()
    }
}

impl TryFrom<char> for Operator {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operator::Add),
            '*' => Ok(Operator::Multiply),
            _ => Err(Error::ParseError(format!(
                "invalid char for Operator: {}",
                value
            ))),
        }
    }
}

struct MathProblem {
    numbers: Vec<i64>,
    operator: Option<Operator>,
}

impl MathProblem {
    fn new() -> MathProblem {
        MathProblem {
            numbers: Vec::new(),
            operator: None,
        }
    }

    fn add_digit_at(&mut self, digit: i8, index: usize) {
        // Numbers can arrive out of order (e.g. if the last number has more sig digs).
        // Push 0s into earlier positions as placeholders.
        while index >= self.numbers.len() {
            self.numbers.push(0);
        }
        self.numbers[index] = (self.numbers[index] * 10) + digit as i64;
    }

    fn set_operator_if_unset(&mut self, operator: Operator) -> Result<(), Error> {
        if self.operator.is_some() {
            Err(Error::LogicError(format!(
                "tried to set operator to {:?}, but operator already set to {:?}",
                operator,
                self.operator.unwrap()
            )))
        } else {
            self.operator = Some(operator);
            Ok(())
        }
    }

    fn solve(&self) -> i64 {
        match self.operator {
            Some(Operator::Add) => self
                .numbers
                .iter()
                .copied()
                .reduce(i64::strict_add)
                .unwrap_or(0),
            Some(Operator::Multiply) => self
                .numbers
                .iter()
                .copied()
                .reduce(i64::strict_mul)
                .unwrap_or(0),
            None => 0,
        }
    }
}

fn parse_math_sheet(input: &str) -> Result<Vec<MathProblem>, Error> {
    let mut problems = Vec::new();
    for line in input.lines() {
        for (i, term) in line.split_whitespace().enumerate() {
            let problem = match problems.get_mut(i) {
                Some(p) => p,
                None => {
                    problems.push(MathProblem::new());
                    problems.get_mut(i).unwrap()
                }
            };
            let number_result = term.parse::<i64>();
            if let Ok(number) = number_result {
                problem.numbers.push(number);
                continue;
            }
            if let Ok(operator) = term.parse::<Operator>() {
                problem.set_operator_if_unset(operator)?;
                continue;
            }
            return Err(Error::ParseError(format!(
                "could not parse {} as either a number or operator",
                term
            )));
        }
    }
    if problems
        .iter()
        .any(|p| p.numbers.is_empty() || p.operator.is_none())
    {
        return Err(Error::ParseError(
            "missing numbers or operator for at least one problem".to_string(),
        ));
    }
    Ok(problems)
}

fn parse_char_matrix(input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let max_line_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    for line in lines.iter_mut() {
        while line.len() < max_line_len {
            line.push(' ');
        }
    }
    lines
}

fn parse_cephalopod_math_sheet(input: &str) -> Result<Vec<MathProblem>, Error> {
    let mut problems = Vec::new();
    let char_lines = parse_char_matrix(input);
    if char_lines.is_empty() {
        return Ok(Vec::new());
    }
    let mut problem = MathProblem::new();
    let mut num_idx = 0;
    for char_idx in 0..char_lines[0].len() {
        let mut is_all_whitespace = true;
        for (dig_idx, _) in char_lines.iter().enumerate() {
            let line = &char_lines[dig_idx];
            let char = line[char_idx];
            if char.is_whitespace() {
                continue;
            }
            if let Some(digit) = char.to_digit(10) {
                problem.add_digit_at(digit as i8, num_idx);
                is_all_whitespace = false;
                continue;
            } else if let Ok(operator) = char.try_into() {
                problem.set_operator_if_unset(operator)?;
                is_all_whitespace = false;
                continue;
            }
            return Err(Error::ParseError(format!(
                "received unexpected char {} at line: {}, column: {}",
                char, dig_idx, char_idx
            )));
        }
        if is_all_whitespace {
            problems.push(problem);
            num_idx = 0;
            problem = MathProblem::new();
        } else {
            num_idx += 1;
        }
    }
    if !problem.numbers.is_empty() {
        problems.push(problem);
    }
    Ok(problems)
}

fn solve_math_sheet(problems: &[MathProblem]) -> Vec<i64> {
    problems.iter().map(MathProblem::solve).collect()
}

pub fn solve_and_sum_math_sheet(input: &str) -> Result<i64, Error> {
    let problems = parse_math_sheet(input)?;
    let solutions = solve_math_sheet(&problems);
    log::info!("Solved {} math problems", solutions.len());

    Ok(solutions.into_iter().reduce(i64::strict_add).unwrap_or(0))
}

pub fn solve_and_sum_cephalopod_math_sheet(input: &str) -> Result<i64, Error> {
    let problems = parse_cephalopod_math_sheet(input)?;
    let solutions = solve_math_sheet(&problems);
    log::info!("Solved {} math problems", solutions.len());

    Ok(solutions.into_iter().reduce(i64::strict_add).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_math_sheet_small() {
        let result = solve_math_sheet(
            &parse_math_sheet(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ",
            )
            .unwrap(),
        );

        assert_eq!(result, vec!(33210, 490, 4243455, 401))
    }

    #[test]
    fn solve_and_sum_math_sheet_small() {
        let result = solve_and_sum_math_sheet(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4277556);
    }

    #[test]
    fn solve_cephalopod_math_sheet_small() {
        let result = solve_math_sheet(
            &parse_cephalopod_math_sheet(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ",
            )
            .unwrap(),
        );

        assert_eq!(result, vec!(8544, 625, 3253600, 1058))
    }

    #[test]
    fn solve_and_sum_cephalopod_math_sheet_small() {
        let result = solve_and_sum_cephalopod_math_sheet(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3263827);
    }
}
