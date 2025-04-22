use super::Piece;
use crate::Orientation;

#[derive(Debug)]
pub struct Ramp(Orientation);

impl Piece for Ramp {
    fn symbol(&self) -> char {
        match self.0 {
            Orientation::Left => '/',
            Orientation::Right => '\\',
        }
    }

    fn from_symbol(symbol: char) -> Option<Box<dyn Piece>>
    where
        Self: Sized,
    {
        Some(Box::new(match symbol {
            '/' => Ramp(Orientation::Left),
            '\\' => Ramp(Orientation::Right),
            _ => return None,
        }))
    }
}

#[derive(Debug)]
pub struct Crossover;

impl Piece for Crossover {
    fn symbol(&self) -> char {
        'X'
    }

    fn from_symbol(symbol: char) -> Option<Box<dyn Piece>>
    where
        Self: Sized,
    {
        Some(Box::new(match symbol {
            'X' | 'x' => Self,
            _ => return None,
        }))
    }
}

#[derive(Debug)]
pub struct Bit(Orientation);

impl Piece for Bit {
    fn symbol(&self) -> char {
        match self.0 {
            Orientation::Left => '<',
            Orientation::Right => '>',
        }
    }

    fn from_symbol(symbol: char) -> Option<Box<dyn Piece>>
    where
        Self: Sized,
    {
        Some(Box::new(match symbol {
            '<' => Bit(Orientation::Left),
            '>' => Bit(Orientation::Right),
            _ => return None,
        }))
    }
}

#[derive(Debug)]
pub struct Interceptor;

impl Piece for Interceptor {
    fn symbol(&self) -> char {
        'U'
    }

    fn from_symbol(symbol: char) -> Option<Box<dyn Piece>>
    where
        Self: Sized,
    {
        Some(Box::new(match symbol {
            'U' | 'u' => Interceptor,
            _ => return None,
        }))
    }
}

#[derive(Debug)]
pub struct GearBit(Orientation);

impl Piece for GearBit {
    fn symbol(&self) -> char {
        match self.0 {
            Orientation::Left => '{',
            Orientation::Right => '}',
        }
    }

    fn from_symbol(symbol: char) -> Option<Box<dyn Piece>>
    where
        Self: Sized,
    {
        Some(Box::new(match symbol {
            '{' => GearBit(Orientation::Left),
            '}' => GearBit(Orientation::Right),
            _ => return None,
        }))
    }
}

#[derive(Debug)]
pub struct Gear;

impl Piece for Gear {
    fn symbol(&self) -> char {
        '*'
    }

    fn from_symbol(symbol: char) -> Option<Box<dyn Piece>>
    where
        Self: Sized,
    {
        Some(Box::new(match symbol {
            '*' => Gear,
            _ => return None,
        }))
    }
}
