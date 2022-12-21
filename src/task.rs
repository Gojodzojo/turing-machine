use core::fmt;

use crate::constants::{DEFAULT_STATE, DEFAULT_TASK_CHAR, DEFAULT_TASK_DIRECTION};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Stop,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDirectionError;

impl TryFrom<char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if c == '-' {
            return Ok(Direction::Left);
        } else if c == '+' {
            return Ok(Direction::Right);
        } else if c == '0' {
            return Ok(Direction::Stop);
        }

        Err(ParseDirectionError)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Direction::Left => '-',
            Direction::Right => '+',
            Direction::Stop => '0',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub state: usize,
    pub character: char,
    pub direction: Direction,
}

impl Task {
    pub fn new() -> Self {
        Self {
            state: DEFAULT_STATE,
            character: DEFAULT_TASK_CHAR,
            direction: DEFAULT_TASK_DIRECTION,
        }
    }
}
