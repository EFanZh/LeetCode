pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let negative = nums.partition_point(|&x| x < 0);
        let zeros = nums[negative..].partition_point(|&x| x == 0);

        negative.max(nums.len() - negative - zeros) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_count(nums: Vec<i32>) -> i32 {
        Self::maximum_count(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
