pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn concat(x: u32, y: u32) -> u32 {
        (x << (32 - y.leading_zeros())) | y
    }

    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        nums.sort_unstable_by(|&lhs, &rhs| Self::concat(rhs, lhs).cmp(&Self::concat(lhs, rhs)));

        nums.iter().copied().fold(0, Self::concat).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_good_number(nums: Vec<i32>) -> i32 {
        Self::max_good_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
