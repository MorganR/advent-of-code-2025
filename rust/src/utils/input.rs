// Input parsing utilities for Advent of Code solutions
// Add parsing helpers as needed when implementing days

use thiserror::Error as ThisError;

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("logic error: {0}")]
    LogicError(String),
}
