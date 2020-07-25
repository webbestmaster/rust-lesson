use std::cell::RefCell;

#[derive(Debug)]
pub struct Man {
    pub age: u32,
    pub about: RefCell<String>,
}

