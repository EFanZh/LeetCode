// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

pub struct SmallestInfiniteSet {
    front: BinaryHeap<Reverse<u32>>,
    front_values: HashSet<u32>,
    back: u32,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            front: BinaryHeap::new(),
            front_values: HashSet::new(),
            back: 0,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(value)) = self.front.pop() {
            if value < self.back {
                self.front_values.remove(&value);

                return value as _;
            }

            self.front.clear();
        }

        self.back += 1;

        self.back as _
    }

    fn add_back(&mut self, num: i32) {
        let num = num as u32;

        match num.cmp(&self.back) {
            Ordering::Less => {
                if self.front_values.insert(num) {
                    self.front.push(Reverse(num));
                }
            }
            Ordering::Equal => {
                self.front_values.remove(&num);

                loop {
                    self.back -= 1;

                    if !self.front_values.remove(&self.back) {
                        break;
                    }
                }
            }
            Ordering::Greater => {}
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::SmallestInfiniteSet for SmallestInfiniteSet {
    fn new() -> Self {
        Self::new()
    }

    fn pop_smallest(&mut self) -> i32 {
        self.pop_smallest()
    }

    fn add_back(&mut self, num: i32) {
        self.add_back(num);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SmallestInfiniteSet>();
    }
}
