pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &num| acc | num)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_xor(nums: Vec<i32>) -> i32 {
        Self::maximum_xor(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
