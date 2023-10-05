pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::unnecessary_lazy_evaluations)] // Not supported by LeetCode.
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut cache = vec![0; nums.len()];
        let mut result = 0;

        for (i, num) in nums.iter().enumerate().rev() {
            let max_length = cache
                .iter()
                .enumerate()
                .skip(i)
                .filter(|&(j, _)| nums[j] > *num)
                .map(|(_, &length)| length)
                .max()
                .map_or(1, |x| x + 1);

            cache[i] = max_length;

            if max_length > result {
                result = max_length;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::length_of_lis(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
