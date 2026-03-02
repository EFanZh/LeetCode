pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = [0_usize; (128 / usize::BITS) as _];
        let max_distinct = nums
            .iter()
            .rev()
            .take_while(|&&num| {
                let num = num.cast_unsigned() & 127;
                let bucket = &mut seen[(num / usize::BITS) as usize];
                let bit = 1 << (num % usize::BITS);

                if *bucket & bit == 0 {
                    *bucket |= bit;

                    true
                } else {
                    false
                }
            })
            .count();

        (nums.len() - max_distinct).div_ceil(3) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32 {
        Self::minimum_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
