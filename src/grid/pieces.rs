use std::fmt::Display;

use crate::Orientation;

#[derive(Debug)]
pub enum Piece {
    Ramp(Orientation),
    Bit(Orientation),
    GearBit(Orientation),
    Crossover,
    Interceptor,
    Gear,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl Piece {
    pub(crate) fn symbol(&self) -> char {
        match self {
            Self::Ramp(Orientation::Left) => '/',
            Self::Ramp(Orientation::Right) => '\\',
            Self::Bit(Orientation::Left) => '<',
            Self::Bit(Orientation::Right) => '>',
            Self::GearBit(Orientation::Left) => '{',
            Self::GearBit(Orientation::Right) => '}',
            Self::Crossover => 'X',
            Self::Interceptor => 'U',
            Self::Gear => '*',
        }
    }

    pub(crate) fn from_symbol(symbol: char) -> Option<Self> {
        Some(match symbol {
            '/' => Self::Ramp(Orientation::Left),
            '\\' => Self::Ramp(Orientation::Right),
            '<' => Self::Bit(Orientation::Left),
            '>' => Self::Bit(Orientation::Right),
            '{' => Self::GearBit(Orientation::Left),
            '}' => Self::GearBit(Orientation::Right),
            'X' | 'x' => Self::Crossover,
            'U' | 'u' => Self::Interceptor,
            '*' => Self::Gear,
            _ => return None,
        })
    }
}
