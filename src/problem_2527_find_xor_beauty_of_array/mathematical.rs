pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |xor, &num| xor ^ num)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn xor_beauty(nums: Vec<i32>) -> i32 {
        Self::xor_beauty(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
