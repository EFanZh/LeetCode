pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as u8;
        let mut seen = 0_u128;
        let mut pairs = 0;

        for num in nums {
            let probe = 1 << (num as u8);

            pairs += u8::from((seen & probe) != 0);
            seen ^= probe;
        }

        vec![i32::from(pairs), i32::from(n - pairs * 2)]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        Self::number_of_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
