pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn three_way_partition(nums: &mut [u32], key: u32) -> (usize, usize, u64) {
        let mut less_count = 0;
        let mut left = 0;
        let mut right = nums.len();
        let mut left_sum = 0;

        while left < right {
            let num = nums[left];

            match num.cmp(&key) {
                Ordering::Less => {
                    nums[less_count] = num;
                    less_count += 1;
                    left_sum += u64::from(num);
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    right -= 1;
                    nums.swap(left, right);

                    continue;
                }
            }

            left += 1;
        }

        nums[less_count..left].fill(key);

        (less_count, left, left_sum)
    }

    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let n = n as u32 as usize;
        let mut batteries = batteries.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let m = batteries.len();
        let split = m - n;
        let mut left = 0;
        let mut right = batteries.len();
        let mut left_sum = 0;

        while left < right {
            let window = &mut batteries[left..right];
            let middle = window.len() / 2;
            let key = u64::from(window[middle]);
            let (middle_start, middle_end, window_left_sum) = Self::three_way_partition(window, key as _);
            let middle = middle.clamp(middle_start, middle_end - 1);
            let candidate_right = left + middle;
            let candidate_left = candidate_right + 1;
            let candidate_left_sum = left_sum + window_left_sum + key * (middle - middle_start + 1) as u64;

            if candidate_left < split + 2 || key * (candidate_left - split) as u64 <= candidate_left_sum {
                left = candidate_left;
                left_sum = candidate_left_sum;
            } else {
                right = candidate_right;
            }
        }

        (left_sum / (left - split) as u64) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        Self::max_run_time(n, batteries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
