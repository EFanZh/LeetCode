pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let max_num = nums.iter().fold(0, |x, &y| x.max(y));
        let mut counts = vec![0_u32; max_num as usize].into_boxed_slice();

        for num in nums {
            counts[num as usize - 1] += 1;
        }

        let mut total_count = 0;

        for count in &mut *counts {
            total_count += *count;
            *count = total_count;
        }

        let mut result = 0_u64;
        let mut left_count = 0;

        for (i, &right_count) in counts.iter().enumerate() {
            if right_count != left_count {
                let mut sum = 0;
                let mut prev_count = left_count;
                let mut quotient = 1;

                if let Some(window) = counts.get(i * 2..) {
                    for &count in window.iter().step_by(i + 1) {
                        sum += u64::from(count - prev_count) * quotient;
                        prev_count = count;
                        quotient += 1;
                    }
                }

                sum += u64::from(total_count - prev_count) * quotient;
                result += u64::from(right_count - left_count) * sum;
                left_count = right_count;
            }
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        Self::sum_of_floored_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
