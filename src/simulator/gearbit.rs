use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    rc::Rc,
};

use crate::Orientation;

pub struct GearBit {
    shared_orientation: Rc<Cell<Orientation>>,
    pub left: Option<Rc<RefCell<GearBit>>>,
    pub right: Option<Rc<RefCell<GearBit>>>,
}

impl Debug for GearBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn inner(
            gearbit: &GearBit,
            f: &mut std::fmt::Formatter,
            seen: &mut Vec<*mut Orientation>,
        ) -> std::fmt::Result {
            seen.push(gearbit.shared_orientation.as_ptr());

            write!(
                f,
                "GearBit {{ orientation: [{}]={:?}",
                seen.iter()
                    .position(|x| x == &gearbit.shared_orientation.as_ptr())
                    .unwrap(),
                gearbit.shared_orientation.get(),
            )?;

            if let Some(left) = &gearbit.left {
                write!(f, ", left: ")?;
                inner(&left.borrow(), f, seen)?;
            }

            if let Some(right) = &gearbit.right {
                write!(f, ", right: ")?;
                inner(&right.borrow(), f, seen)?;
            }

            write!(f, " }}")
        }

        inner(self, f, &mut Vec::new())
    }
}

impl GearBit {
    pub fn new(orientation: Orientation) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            left: None,
            right: None,
            shared_orientation: Rc::new(Cell::new(orientation)),
        }))
    }

    pub fn new_shared(&self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            left: None,
            right: None,
            shared_orientation: self.shared_orientation.clone(),
        }))
    }

    pub fn trigger(&mut self) -> Option<Rc<RefCell<GearBit>>> {
        let new_orientation = match self.shared_orientation.get() {
            Orientation::Left => Orientation::Right,
            Orientation::Right => Orientation::Left,
        };
        self.shared_orientation.set(new_orientation);
        match new_orientation {
            Orientation::Left => self.left.clone(),
            Orientation::Right => self.right.clone(),
        }
    }

    pub fn orientation(&self) -> Orientation {
        self.shared_orientation.get()
    }
}
