pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn partition_by(nums: &mut [i32], mut f: impl FnMut(i32) -> bool) {
        let mut iter = nums.iter_mut();

        'outer: loop {
            let left = loop {
                if let Some(left) = iter.next() {
                    if !f(*left) {
                        break left;
                    }
                } else {
                    break 'outer;
                }
            };

            loop {
                if let Some(right) = iter.next_back() {
                    if f(*right) {
                        mem::swap(left, right);

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }
    }

    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        Self::partition_by(&mut nums, |x| x % 2 == 0);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array_by_parity(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
