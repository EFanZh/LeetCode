pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let mut left_count = 0;
        let mut right_sum = nums.iter().fold(0_u64, |sum, &x| sum + u64::from(x as u32));
        let mut right_count = nums.len() as u64;
        let mut min_diff = u64::MAX;
        let mut min_diff_index = 0;

        for num in nums {
            let num = u64::from(num as u32);

            left_count += 1;
            right_count -= 1;
            left_sum += num;
            right_sum -= num;

            let left_average = left_sum / left_count;
            let right_average = if right_count == 0 { 0 } else { right_sum / right_count };

            let diff = left_average.abs_diff(right_average);

            if diff < min_diff {
                min_diff = diff;
                min_diff_index = left_count;

                if diff == 0 {
                    break;
                }
            }
        }

        (min_diff_index - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        Self::minimum_average_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
