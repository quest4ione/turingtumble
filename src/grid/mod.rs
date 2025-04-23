mod pieces;
use std::{collections::HashMap, fmt::Display};

use pieces::Piece;

#[derive(Default)]
pub struct Grid {
    pub grid: HashMap<(i64, u64), Piece>,
    pub left_start: Option<i64>,
    pub right_start: Option<i64>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // calculate display range
        let mut x_range: (i64, i64) = (0, 0);
        let mut y_range: (u64, u64) = (0, 0);
        for (x, y) in self.grid.keys() {
            x_range = (x_range.0.min(*x), x_range.1.max(*x));
            y_range = (y_range.0.min(*y), y_range.1.max(*y));
        }

        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                write!(
                    f,
                    "{}",
                    match self.grid.get(&(x, y)) {
                        Some(piece) => piece.symbol(),
                        None => ' ',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_text(text: String) -> Self {
        let mut grid: HashMap<(usize, usize), Piece> = HashMap::new();
        let mut top_pieces_x: Vec<usize> = Vec::new();
        let mut max_width: usize = 0;

        // convert symbol into piece
        for (y, row) in text.split("\n").enumerate() {
            for (x, symbol) in row.to_string().chars().enumerate() {
                max_width = max_width.max(x);

                if let Some(piece) = Piece::from_symbol(symbol) {
                    if y == 0 {
                        top_pieces_x.push(x);
                    }
                    grid.insert((x, y), piece);
                }
            }
        }

        let mut center_x = (max_width / 2) as i64;

        // decide upon left/right start positions (and center_x if there are 2)
        let (left_start, right_start) = match top_pieces_x.len() {
            0 => (None, None),
            1 => {
                // decide between left or right based on center
                let x = *top_pieces_x.first().unwrap() as i64;
                if x > center_x {
                    (None, Some(x - center_x))
                } else {
                    (Some(x - center_x), None)
                }
            }
            2.. => {
                let left = *top_pieces_x.first().unwrap() as i64;
                let right = *top_pieces_x.last().unwrap() as i64;
                center_x = (left + right) / 2;
                (Some(left - center_x), Some(right - center_x))
            }
        };

        // offset grid positions so they originate from the center
        let grid = HashMap::from_iter(
            grid.into_iter()
                .map(|((x, y), piece)| ((x as i64 - center_x, y as u64), piece)),
        );
        Self {
            grid,
            left_start,
            right_start,
        }
    }
}
