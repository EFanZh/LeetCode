pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let mut bits = 0;

        for num in nums {
            if num.cast_unsigned().is_power_of_two() {
                bits |= num;
            }
        }

        let x = !bits;

        x & x.wrapping_neg()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_impossible_or(nums: Vec<i32>) -> i32 {
        Self::min_impossible_or(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
