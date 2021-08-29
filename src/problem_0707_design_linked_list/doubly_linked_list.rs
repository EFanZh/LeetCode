// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    prev: Cell<Weak<Self>>,
    next: Cell<Option<Rc<Self>>>,
}

pub struct MyLinkedList {
    head: Option<Rc<Node>>,
    tail: Weak<Node>,
    length: i32,
}

fn clone_cell_inner<T: Default + Clone>(cell: &Cell<T>) -> T {
    let temp = cell.take();
    let result = temp.clone();

    cell.set(temp);

    result
}

impl MyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: Weak::new(),
            length: 0,
        }
    }

    fn get_node(&self, index: i32) -> Option<Rc<Node>> {
        if index < self.length {
            if index < self.length / 2 {
                let mut node = self.head.clone().unwrap();

                for _ in 0..index {
                    node = clone_cell_inner(&node.next).unwrap();
                }

                Some(node)
            } else {
                let mut node = self.tail.upgrade().unwrap();

                for _ in 0..(self.length - 1 - index) {
                    node = clone_cell_inner(&node.prev).upgrade().unwrap();
                }

                Some(node)
            }
        } else {
            None
        }
    }

    fn get(&self, index: i32) -> i32 {
        self.get_node(index).map_or(-1, |node| node.value)
    }

    fn add_empty(&mut self, val: i32) {
        let node = Rc::new(Node {
            value: val,
            prev: Cell::default(),
            next: Cell::default(),
        });

        self.tail = Rc::downgrade(&node);
        self.head = Some(node);
    }

    fn add_at_head(&mut self, val: i32) {
        if let Some(head) = self.head.clone() {
            let node = Rc::new(Node {
                value: val,
                prev: Cell::default(),
                next: Cell::new(Some(Rc::clone(&head))),
            });

            head.prev.set(Rc::downgrade(&node));
            self.head = Some(node);
        } else {
            self.add_empty(val);
        }

        self.length += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        if let Some(tail) = self.tail.upgrade() {
            let node = Rc::new(Node {
                value: val,
                prev: Cell::new(Rc::downgrade(&tail)),
                next: Cell::default(),
            });

            self.tail = Rc::downgrade(&node);
            tail.next.set(Some(node));
        } else {
            self.add_empty(val);
        }

        self.length += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
        } else if index < self.length {
            let next = self.get_node(index).unwrap();
            let prev = clone_cell_inner(&next.prev).upgrade().unwrap();

            let node = Rc::new(Node {
                value: val,
                prev: Cell::new(Rc::downgrade(&prev)),
                next: Cell::new(Some(Rc::clone(&next))),
            });

            next.prev.set(Rc::downgrade(&node));
            prev.next.set(Some(node));

            self.length += 1;
        } else if index == self.length {
            self.add_at_tail(val);
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if let Some(node) = self.get_node(index) {
            if let Some(prev) = clone_cell_inner(&node.prev).upgrade() {
                prev.next.set(clone_cell_inner(&node.next));
            } else {
                self.head = clone_cell_inner(&node.next);
            }

            if let Some(next) = clone_cell_inner(&node.next) {
                next.prev.set(clone_cell_inner(&node.prev));
            } else {
                self.tail = clone_cell_inner(&node.prev);
            }

            self.length -= 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MyLinkedList for MyLinkedList {
    fn new() -> Self {
        Self::new()
    }

    fn get(&self, index: i32) -> i32 {
        self.get(index)
    }

    fn add_at_head(&mut self, val: i32) {
        self.add_at_head(val);
    }

    fn add_at_tail(&mut self, val: i32) {
        self.add_at_tail(val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        self.add_at_index(index, val);
    }

    fn delete_at_index(&mut self, index: i32) {
        self.delete_at_index(index);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MyLinkedList>();
    }
}
