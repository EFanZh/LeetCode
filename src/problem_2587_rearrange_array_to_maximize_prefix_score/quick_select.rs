pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn three_way_partition(nums: &mut [i32], mut left: usize, mut right: usize, key: i32) -> (usize, usize, i64) {
        let mut middle = left;
        let mut left_sum = 0;

        while middle < right {
            let num = nums[middle];

            match key.cmp(&num) {
                Ordering::Less => {
                    nums[left] = num;
                    left_sum += i64::from(num);
                    left += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    right -= 1;
                    nums.swap(middle, right);

                    continue;
                }
            }

            middle += 1;
        }

        nums[left..middle].fill(key);

        (left, middle, left_sum)
    }

    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut left = 0;
        let mut right = nums.len();
        let mut left_sum = 0;

        while left < right {
            let middle = (left + right) / 2;
            let key = nums[middle];
            let (middle_start, middle_end, window_left_sum) = Self::three_way_partition(&mut nums, left, right, key);
            let middle = middle.clamp(middle_start, middle_end - 1);
            let candidate_left = middle + 1;

            let candidate_left_sum =
                left_sum + window_left_sum + i64::from(key) * ((candidate_left - middle_start) as i64);

            if candidate_left_sum > 0 {
                left = candidate_left;
                left_sum = candidate_left_sum;
            } else {
                right = middle;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(nums: Vec<i32>) -> i32 {
        Self::max_score(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
