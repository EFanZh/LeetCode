pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_merged_integer(nums: Vec<i32>) -> i32 {
        nums.as_chunks::<2>()
            .0
            .iter()
            .fold(u32::MAX, |min, &[x, y]| {
                let x = x.cast_unsigned();
                let y = y.cast_unsigned();

                min.min(x * u32::pow(10, y.ilog10() + 1) + y)
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_merged_integer(nums: Vec<i32>) -> i32 {
        Self::min_merged_integer(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
