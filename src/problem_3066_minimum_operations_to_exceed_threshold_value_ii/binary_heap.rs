pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::binary_heap::PeekMut;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k.cast_unsigned();

        let mut queue = nums
            .into_iter()
            .map(|x| Reverse(x.cast_unsigned()))
            .collect::<BinaryHeap<_>>();

        let mut result = 0;

        loop {
            let min = queue.peek_mut().unwrap();

            if min.0 >= k {
                break;
            }

            result += 1;

            let min = PeekMut::pop(min).0;
            let mut top = queue.peek_mut().unwrap();
            let top_ref = &mut top.0;

            *top_ref = top_ref.saturating_add(min * 2);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        Self::min_operations(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
