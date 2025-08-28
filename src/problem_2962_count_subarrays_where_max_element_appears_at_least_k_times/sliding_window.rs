pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut required = k as u32;
        let max = nums.iter().copied().fold(0, u32::max);
        let mut start = 0;
        let mut result = 0;

        for &num in &nums {
            if num == max {
                if required == 1 {
                    loop {
                        let old = nums[start];

                        start += 1;

                        if old == max {
                            break;
                        }
                    }
                } else {
                    required -= 1;
                }
            }

            result += start as i64;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        Self::count_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
