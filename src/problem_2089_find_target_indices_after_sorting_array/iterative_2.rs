pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut state = 0_u16;

        for num in nums {
            state += match num.cmp(&target) {
                Ordering::Less => 1 << 8,
                Ordering::Equal => 1,
                Ordering::Greater => 0,
            };
        }

        let start = state >> 8;
        let length = state & 0xff;
        let end = start + length;

        (start..end).map(i32::from).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::target_indices(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
