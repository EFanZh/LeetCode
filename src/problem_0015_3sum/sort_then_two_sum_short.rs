pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        // Must have at least 3 elements.

        if nums.len() > 2 {
            let nums = {
                let mut temp = nums;

                temp.sort_unstable();

                temp
            };

            for i in 0..nums.len() - 2 {
                let first_value = nums[i];

                if i == 0 || first_value != nums[i - 1] {
                    let target = -first_value;

                    // Do two sum of target `target` from `i + 1` to `nums.len()`.

                    let mut j = i + 1;
                    let mut k = nums.len() - 1;

                    while j < k {
                        let second_value = nums[j];
                        let third_value = nums[k];

                        match (second_value + third_value).cmp(&target) {
                            Ordering::Less => j += 1,
                            Ordering::Equal => {
                                result.push(vec![first_value, second_value, third_value]);

                                // Skip duplicates.

                                loop {
                                    j += 1;

                                    if j >= k || nums[j] != nums[j - 1] {
                                        break;
                                    }
                                }

                                // Skip duplicates.

                                loop {
                                    k -= 1;

                                    if j >= k || nums[k] != nums[k + 1] {
                                        break;
                                    }
                                }
                            }
                            Ordering::Greater => k -= 1,
                        }
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::three_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
