pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len() as u32;

        nums.sort_unstable();
        nums.dedup();

        let mut start = 0;

        for &num in &nums {
            if nums[start] <= num - n as i32 {
                start += 1;
            }
        }

        let n2 = nums.len() as u32;

        drop(nums);

        (n - (n2 - start as u32)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>) -> i32 {
        Self::min_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
