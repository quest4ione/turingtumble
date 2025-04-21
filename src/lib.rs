pub mod grid;
pub mod simulator;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Orientation {
    Left,
    Right,
}
