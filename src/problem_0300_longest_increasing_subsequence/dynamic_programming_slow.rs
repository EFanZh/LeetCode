pub struct Solution {}

impl Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut cache = vec![0; nums.len()];
        let mut result = 0;

        for (i, num) in nums.iter().enumerate().rev() {
            let max_length = cache
                .iter()
                .enumerate()
                .skip(i)
                .filter_map(|(j, length)| if nums[j] > *num { Some(*length) } else { None })
                .max()
                .map_or(1, |x| x + 1);

            cache[i] = max_length;

            if max_length > result {
                result = max_length
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::length_of_lis(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
