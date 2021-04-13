pub struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut cache = nums.clone();

        for length in 2..=n {
            for start in 0..=n - length {
                let diff_1 = nums[start + (length - 1)] - cache[start];
                let diff_2 = nums[start] - cache[start + 1];

                cache[start] = diff_1.max(diff_2)
            }
        }

        cache[0] >= 0
    }
}

impl super::Solution for Solution {
    fn predict_the_winner(nums: Vec<i32>) -> bool {
        Self::predict_the_winner(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
