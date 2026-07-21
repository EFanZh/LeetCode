pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();

        nums.resize(n * 2, 0);

        let (left, right) = nums.split_at_mut(n);

        right
            .iter_mut()
            .zip(left.iter().rev())
            .for_each(|(target, &value)| *target = value);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
        Self::concat_with_reverse(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
