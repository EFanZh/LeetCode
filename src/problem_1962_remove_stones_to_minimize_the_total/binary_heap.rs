pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut queue = BinaryHeap::from(piles);

        for _ in 0..k {
            let num = &mut *queue.peek_mut().unwrap();

            *num = (*num + 1) / 2;
        }

        queue.iter().sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        Self::min_stone_sum(piles, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
