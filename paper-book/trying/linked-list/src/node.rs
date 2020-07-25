// use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn get_first(&self) -> &Node {
        let next = &self.next;

        return match next {
            Some(ref next) => next.get_first(),
            None => &self,
        };
    }

    pub fn set_next(&mut self, node: Option<Box<Node>>) {
        self.next = node;
    }
}
