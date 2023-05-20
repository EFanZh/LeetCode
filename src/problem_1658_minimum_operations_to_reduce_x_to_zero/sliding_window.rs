pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let nums = nums.as_slice();
        let n = nums.len();
        let x = x as u32;
        let mut sum = 0;
        let mut j = n;

        let mut max_excluded = loop {
            j = j.wrapping_sub(1);

            if let Some(&num) = nums.get(j) {
                sum += num as u32;

                break match sum.cmp(&x) {
                    Ordering::Less => continue,
                    Ordering::Equal => j,
                    Ordering::Greater => n + 1,
                };
            }

            return -1;
        };

        let mut i = 0;

        while let Some(&right) = nums.get(j) {
            j += 1;
            sum -= right as u32;

            loop {
                match sum.cmp(&x) {
                    Ordering::Less => {
                        sum += nums[i] as u32;
                        i += 1;

                        continue;
                    }
                    Ordering::Equal => {
                        let excluded = j - i;

                        max_excluded = if max_excluded == n + 1 {
                            excluded
                        } else {
                            max_excluded.max(excluded)
                        };
                    }
                    Ordering::Greater => {}
                }

                break;
            }
        }

        n.wrapping_sub(max_excluded) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        Self::min_operations(nums, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
