pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let diff = k * 2;

        nums.sort_unstable();

        let mut start = 0;

        for num in &nums {
            start += usize::from(num - nums[start] > diff);
        }

        (nums.len() - start) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        Self::maximum_beauty(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
