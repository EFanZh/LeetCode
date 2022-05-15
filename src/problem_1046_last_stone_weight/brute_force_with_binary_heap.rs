pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        while let Some(weight_1) = heap.pop() {
            if let Some(mut weight_2) = heap.peek_mut() {
                if *weight_2 == weight_1 {
                    PeekMut::pop(weight_2);
                } else {
                    *weight_2 = weight_1 - *weight_2;
                }
            } else {
                return weight_1;
            }
        }

        0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn last_stone_weight(stones: Vec<i32>) -> i32 {
        Self::last_stone_weight(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
