pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut nums = nums.as_slice();
        let mut result = i32::MIN;

        while let [first, rest @ .., last] = nums {
            result = result.max(first + last);

            nums = rest;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_pair_sum(nums: Vec<i32>) -> i32 {
        Self::min_pair_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
