pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        let mut total_xor = 0;

        for num in &nums {
            total_xor ^= num;
        }

        total_xor == 0 || nums.len() % 2 == 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn xor_game(nums: Vec<i32>) -> bool {
        Self::xor_game(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
