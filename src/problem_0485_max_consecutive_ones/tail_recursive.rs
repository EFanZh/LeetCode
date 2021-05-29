pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn zero_before(nums: &[i32], i: usize, result: usize) -> usize {
        match nums.get(i) {
            None => result,
            Some(0) => Self::zero_before(nums, i + 1, result),
            Some(_) => Self::one_before(nums, i + 1, i, result),
        }
    }

    fn one_before(nums: &[i32], i: usize, start: usize, result: usize) -> usize {
        match nums.get(i) {
            None => result.max(i - start),
            Some(0) => Self::zero_before(nums, i + 1, result.max(i - start)),
            Some(_) => Self::one_before(nums, i + 1, start, result),
        }
    }

    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        Self::zero_before(&nums, 0, 0) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        Self::find_max_consecutive_ones(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
