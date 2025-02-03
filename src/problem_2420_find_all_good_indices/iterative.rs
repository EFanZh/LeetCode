pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            let mut nums = nums;
            let n = nums.len() as i32;

            nums.clear();
            nums.extend(k..n - k);

            return nums;
        }

        let nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let k = k as u32 as usize;
        let mut left_length = 0;
        let mut left_prev = 0;

        for &num in nums[..k - 1].iter().rev() {
            if num < left_prev {
                break;
            }

            left_prev = num;
            left_length += 1;
        }

        left_prev = nums.get(k.wrapping_sub(2)).copied().unwrap_or(0);

        let mut right_length = 0;
        let mut right_prev = u32::MAX;

        for &num in nums[k + 1..k * 2].iter().rev() {
            if right_prev < num {
                break;
            }

            right_prev = num;
            right_length += 1;
        }

        right_prev = nums[k * 2 - 1];

        (k..)
            .zip(nums[k - 1..].iter().zip(&nums[k * 2..]))
            .filter_map(|(i, (&left_num, &right_num))| {
                left_length = if left_prev < left_num { 1 } else { left_length + 1 };
                left_prev = left_num;
                right_length = if right_num < right_prev { 1 } else { right_length + 1 };
                right_prev = right_num;

                (left_length >= k && right_length >= k).then_some(i as _)
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::good_indices(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
