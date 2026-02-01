pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for num in &mut nums {
            *num = if *num & 1 == 0 {
                -1
            } else {
                *num ^ (1 << (num.trailing_ones() - 1))
            };
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        Self::min_bitwise_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
