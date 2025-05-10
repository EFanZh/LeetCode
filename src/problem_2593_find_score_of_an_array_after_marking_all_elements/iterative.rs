pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let mut indices = (0..nums.len() as u32).collect::<Vec<_>>();

        indices.sort_unstable_by_key(|&index| (nums[index as usize], index));

        let mut result = 0;

        for index in indices {
            let index = index as usize;
            let num = mem::take(&mut nums[index]);

            if num != 0 {
                result += u64::from(num);

                if let Some(left) = nums.get_mut(index.wrapping_sub(1)) {
                    *left = 0;
                }

                if let Some(right) = nums.get_mut(index + 1) {
                    *right = 0;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_score(nums: Vec<i32>) -> i64 {
        Self::find_score(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
