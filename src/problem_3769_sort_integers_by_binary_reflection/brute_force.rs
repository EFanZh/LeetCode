pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        nums.sort_unstable_by_key(|num| {
            let num = num.cast_unsigned();

            ((num << num.leading_zeros()).reverse_bits(), num)
        });

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_by_reflection(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
