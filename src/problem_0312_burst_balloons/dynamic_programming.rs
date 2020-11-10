pub struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len() + 1;
        let mut cache = vec![0; n * (n - 1) + 1];

        for length in 1..n {
            let (cache, row) = cache[..n * (length + 1) - length].split_at_mut(n * length);

            for (start, value) in row.iter_mut().enumerate() {
                for (left_length, &num) in nums[start..start + length].iter().enumerate() {
                    let right_length = length - 1 - left_length;
                    let left_coins = cache[n * left_length + start];
                    let right_coins = cache[n * right_length + start + left_length + 1];
                    let left = nums.get(start.wrapping_sub(1)).copied().unwrap_or(1);
                    let right = nums.get(start + length).copied().unwrap_or(1);

                    *value = (*value).max(left_coins + right_coins + left * num * right);
                }
            }
        }

        *cache.last().unwrap()
    }
}

impl super::Solution for Solution {
    fn max_coins(nums: Vec<i32>) -> i32 {
        Self::max_coins(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
