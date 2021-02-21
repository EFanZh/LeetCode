pub struct Solution;

impl Solution {
    fn helper(nums: &[i32], m: i32, max_sum: i32) -> bool {
        let mut sum = 0;
        let mut count = 1;

        for &num in nums {
            sum += num;

            if sum > max_sum {
                count += 1;

                if count > m {
                    return false;
                }

                sum = num;
            }
        }

        true
    }

    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut sum = 0;
        let mut max = 0;

        for &num in &nums {
            sum += num;
            max = max.max(num);
        }

        let mut left = ((sum + (m - 1)) / m).max(max);
        let mut right = sum;

        while left != right {
            let middle = (left + right) / 2;

            if Self::helper(&nums, m, middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left
    }
}

impl super::Solution for Solution {
    fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        Self::split_array(nums, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
