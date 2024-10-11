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

    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut capacity = capacity;

        capacity
            .iter_mut()
            .zip(rocks)
            .for_each(|(capacity, rocks)| *capacity -= rocks);

        let mut capacity = capacity.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let additional_rocks = u64::from(additional_rocks as u32);
        let mut left = 0;
        let mut right = capacity.len();
        let mut left_sum = 0;

        while left < right {
            let window = &mut capacity[left..right];
            let middle = window.len() / 2;
            let key = window[middle];
            let (middle_start, middle_end, window_left_sum) = Self::three_way_partition(window, key);
            let middle = middle.clamp(middle_start, middle_end - 1);
            let candidate_right = left + middle;
            let candidate_left = candidate_right + 1;
            let candidate_left_sum = left_sum + window_left_sum + u64::from(key) * (middle - middle_start + 1) as u64;

            if candidate_left_sum <= additional_rocks {
                left = candidate_left;
                left_sum = candidate_left_sum;
            } else {
                right = candidate_right;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        Self::maximum_bags(capacity, rocks, additional_rocks)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
