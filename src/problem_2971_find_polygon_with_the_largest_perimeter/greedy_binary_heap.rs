pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;
use std::collections::binary_heap::PeekMut;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut heap = nums.into_iter().map(i32::cast_unsigned).collect::<BinaryHeap<_>>();
        let mut sum = heap.iter().copied().map(u64::from).sum::<u64>();

        loop {
            if heap.len() > 2 {
                let peek_mut = heap.peek_mut().unwrap();
                let top = u64::from(*peek_mut);

                if top * 2 < sum {
                    return sum.cast_signed();
                }

                PeekMut::pop(peek_mut);

                sum -= top;
            } else {
                return -1;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_perimeter(nums: Vec<i32>) -> i64 {
        Self::largest_perimeter(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
