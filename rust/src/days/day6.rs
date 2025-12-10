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
            return Err(Error::ParseError(format!("Operator string must have length 1, received: {}", s)));
        }
        let char = s.chars().next().unwrap();

        match char {
            '+' => Ok(Operator::Add),
            '*' => Ok(Operator::Multiply),
            _ => Err(Error::ParseError(format!("Invalid first char for operator: {}", char)))
        }
    }
}

struct MathProblem {
    numbers: Vec<i64>,
    operator: Option<Operator>,
}

impl MathProblem {
    fn new() -> MathProblem {
        MathProblem { numbers: Vec::new(), operator: None }
    }

    fn set_operator_if_unset(&mut self, operator: Operator) -> Result<(), Error> {
        if self.operator.is_some() {
            Err(Error::LogicError(format!("tried to set operator to {:?}, but operator already set to {:?}", operator, self.operator.unwrap())))
        } else {
            self.operator = Some(operator);
            Ok(())
        }
    }

    fn solve(&self) -> i64 {
        match self.operator {
            Some(Operator::Add) => self.numbers.iter().map(|&n| n).reduce(i64::strict_add).unwrap_or(0),
            Some(Operator::Multiply) => self.numbers.iter().map(|&n| n).reduce(i64::strict_mul).unwrap_or(0),
            None => 0
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
            return Err(Error::ParseError(format!("could not parse {} as either a number or operator", term)))
        }
    }
    if problems.iter().any(|p| p.numbers.is_empty() || p.operator.is_none()) {
        return Err(Error::ParseError("missing numbers or operator for at least one problem".to_string()));
    }
    Ok(problems)
}

fn solve_math_sheet(input: &str) -> Result<Vec<i64>, Error> {
    let problems = parse_math_sheet(input)?;

    Ok(problems.iter().map(MathProblem::solve).collect())
}

pub fn solve_and_sum_math_sheet(input: &str) -> Result<i64, Error> {
    let solutions = solve_math_sheet(input)?;
    log::info!("Solved {} math problems", solutions.len());

    Ok(solutions.into_iter().reduce(i64::strict_add).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_math_sheet_small() {
        let result = solve_math_sheet("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!(33210, 490, 4243455, 401))
    }

    #[test]
    fn solve_and_sum_math_sheet_small() {
        let result = solve_and_sum_math_sheet("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4277556);
    }
}