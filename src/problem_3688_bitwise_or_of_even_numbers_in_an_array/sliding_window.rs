pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        nums.iter()
            .copied()
            .filter(|&num| num & 1 == 0)
            .fold(0, |lhs, rhs| lhs | rhs)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        Self::even_number_bitwise_o_rs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
