// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

struct Item {
    expiration_time: u32,
    token_id: Rc<str>,
    index: Rc<Cell<usize>>,
}

pub struct AuthenticationManager {
    time_to_live: u32,
    heap: Vec<Item>,
    entries: HashMap<Rc<str>, Rc<Cell<usize>>>,
}

impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self {
            time_to_live: time_to_live as _,
            heap: Vec::new(),
            entries: HashMap::new(),
        }
    }

    fn heap_sift_down(&mut self, index: usize, new_expiration_time: u32) {
        let mut index = index;

        loop {
            let left_child_index = index * 2 + 1;

            if let Some(left_child) = self.heap.get(left_child_index) {
                let right_child_index = left_child_index + 1;

                if let Some(right_child) = self.heap.get(right_child_index) {
                    let (child, child_index) = if right_child.expiration_time < left_child.expiration_time {
                        (right_child, right_child_index)
                    } else {
                        (left_child, left_child_index)
                    };

                    if child.expiration_time < new_expiration_time {
                        child.index.set(index);
                        self.heap.swap(index, child_index);
                        index = child_index;
                    } else {
                        break;
                    }
                } else {
                    if left_child.expiration_time < new_expiration_time {
                        left_child.index.set(index);
                        self.heap.swap(index, left_child_index);
                        index = left_child_index;
                    }

                    break;
                }
            } else {
                break;
            }
        }

        self.heap[index].index.set(index);
    }

    fn advance(&mut self, current_time: u32) {
        while let Some(top) = self.heap.first() {
            if top.expiration_time <= current_time {
                let mut last = self.heap.pop().unwrap();

                if let Some(first) = self.heap.first_mut() {
                    mem::swap(first, &mut last);

                    let expiration_time = first.expiration_time;

                    self.heap_sift_down(0, expiration_time);

                    self.entries.remove(last.token_id.as_ref());
                } else {
                    self.entries.remove(last.token_id.as_ref());

                    break;
                }
            } else {
                break;
            }
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        let token_id = Rc::from(token_id);
        let current_time = current_time as u32;
        let index = Rc::new(Cell::new(self.heap.len()));

        self.heap.push(Item {
            expiration_time: current_time + self.time_to_live,
            token_id: Rc::clone(&token_id),
            index: Rc::clone(&index),
        });

        self.entries.insert(token_id, index);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        let current_time = current_time as u32;

        self.advance(current_time);

        if let Some(index) = self.entries.get(token_id.as_str()) {
            let index = index.get();
            let expiration_time = current_time + self.time_to_live;

            self.heap[index].expiration_time = expiration_time;
            self.heap_sift_down(index, expiration_time);
        }
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        let current_time = current_time as u32;

        self.advance(current_time);

        self.heap.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::AuthenticationManager for AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self::new(time_to_live)
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.generate(token_id, current_time);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        self.renew(token_id, current_time);
    }

    fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        self.count_unexpired_tokens(current_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::AuthenticationManager>();
    }
}
