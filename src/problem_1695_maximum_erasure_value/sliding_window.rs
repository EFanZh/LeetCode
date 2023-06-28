pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut set = HashSet::<u16>::new();
        let mut sum = 0_u32;
        let mut result = 0;

        for &new in &nums {
            let new = new as u16;

            sum += u32::from(new);

            if !set.insert(new) {
                loop {
                    let old = nums[start] as u16;

                    start += 1;
                    sum -= u32::from(old);

                    if old == new {
                        break;
                    }

                    set.remove(&old);
                }
            }

            result = result.max(sum);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        Self::maximum_unique_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
