pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut result = 0;
        let mut count = 0;
        let mut min_start = 0;
        let mut max_start = 0;

        for &num in &nums {
            count += u32::from(num % 2 != 0);

            match count.cmp(&k) {
                Ordering::Less => {}
                Ordering::Equal => {
                    while nums[max_start] % 2 == 0 {
                        max_start += 1;
                    }

                    result += max_start - min_start + 1;
                }
                Ordering::Greater => {
                    while nums[min_start] % 2 == 0 {
                        min_start += 1;
                    }

                    count -= 1;
                    min_start += 1;
                    max_start = min_start;

                    while nums[max_start] % 2 == 0 {
                        max_start += 1;
                    }

                    result += max_start - min_start + 1;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        Self::number_of_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
