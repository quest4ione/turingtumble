mod gearbit;
pub use self::gearbit::GearBit;
use crate::Orientation;
use std::{cell::RefCell, rc::Rc};

pub enum State {
    Simulating,
    Intercepted,
}

#[derive(Default)]
pub struct Simulator {
    left_root: Option<Rc<RefCell<GearBit>>>,
    right_root: Option<Rc<RefCell<GearBit>>>,
    curr: Option<Rc<RefCell<GearBit>>>,
}

impl Simulator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_roots(
        left_root: Option<Rc<RefCell<GearBit>>>,
        right_root: Option<Rc<RefCell<GearBit>>>,
    ) -> Self {
        Self {
            left_root,
            right_root,
            curr: None,
        }
    }

    pub fn state(&self) -> State {
        match self.curr {
            Some(_) => State::Simulating,
            None => State::Intercepted,
        }
    }

    pub fn trigger(&mut self, side: Orientation) {
        self.curr = match side {
            Orientation::Left => self.left_root.clone(),
            Orientation::Right => self.right_root.clone(),
        }
    }

    pub fn step(&mut self) -> State {
        self.curr = match &self.curr {
            None => return State::Intercepted,
            Some(curr) => curr.borrow_mut().trigger().clone(),
        };
        self.state()
    }
}
