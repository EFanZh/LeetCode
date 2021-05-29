// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem;

pub struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left.len() == self.right.len() {
            self.left.push(self.right.peek_mut().map_or(num, |mut right| {
                if num <= right.0 {
                    num
                } else {
                    mem::replace(&mut right.0, num)
                }
            }));
        } else {
            self.right.push(Reverse(self.left.peek_mut().map_or(num, |mut left| {
                if num >= *left {
                    num
                } else {
                    mem::replace(&mut left, num)
                }
            })));
        }
    }

    fn find_median(&self) -> f64 {
        if self.left.len() == self.right.len() {
            f64::from(self.left.peek().unwrap() + self.right.peek().unwrap().0) / 2.0
        } else {
            f64::from(*self.left.peek().unwrap())
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MedianFinder for MedianFinder {
    fn new() -> Self {
        Self::new()
    }

    fn add_num(&mut self, num: i32) {
        self.add_num(num);
    }

    fn find_median(&self) -> f64 {
        self.find_median()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MedianFinder>();
    }
}
