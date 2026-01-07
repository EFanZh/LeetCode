pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Item<'a>(&'a mut i32);

impl PartialEq for Item<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item<'_> {}

impl PartialOrd for Item<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        fn ptr_cmp<T>(lhs: *const T, rhs: *const T) -> Ordering {
            lhs.cmp(&rhs)
        }

        i32::cmp(other.0, self.0).then_with(|| ptr_cmp(other.0, self.0))
    }
}

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut nums = nums;
        let k = k.cast_unsigned();
        let mut queue = nums.iter_mut().map(Item).collect::<BinaryHeap<_>>();

        assert!(!queue.is_empty());

        for _ in 0..k {
            *queue.peek_mut().unwrap().0 *= multiplier;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        Self::get_final_state(nums, k, multiplier)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
