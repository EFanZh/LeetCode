pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut nums = nums;

        for i in 0..nums.len() {
            if nums[i] != -1 {
                let mut current = i;
                let mut count = 1;

                loop {
                    let next = mem::replace(&mut nums[current], -1) as _;

                    if next == i {
                        break;
                    }

                    current = next;
                    count += 1;
                }

                result = result.max(count);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn array_nesting(nums: Vec<i32>) -> i32 {
        Self::array_nesting(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
