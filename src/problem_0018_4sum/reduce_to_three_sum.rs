pub struct Solution {}

use std::cmp::Ordering;

fn partition<T, F: FnMut(&T) -> bool>(values: &mut [T], mut f: F) -> (&mut [T], &mut [T]) {
    let mut pivot = 0;

    for i in 0..values.len() {
        if f(&values[i]) {
            values.swap(i, pivot);

            pivot += 1;
        }
    }

    values.split_at_mut(pivot)
}

impl Solution {
    fn partition_and_sort(mut nums: Vec<i32>, max_first_value: i32) -> (Vec<i32>, usize) {
        let (left, right) = partition(nums.as_mut(), |x| *x <= max_first_value);
        let pivot = left.len();

        left.sort_unstable();
        right.sort_unstable();

        (nums, pivot)
    }

    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        // Must have at least 4 elements.

        if nums.len() > 3 {
            let (nums, pivot) = Self::partition_and_sort(nums, target / 4);

            for i in 0..pivot.min(nums.len() - 3) {
                let first_value = nums[i];

                if i == 0 || first_value != nums[i - 1] {
                    // Do three sum of target `target - first_value` from `i + 1` to `nums.len()`.

                    let nums = &nums[i + 1..];
                    let target = target - first_value;

                    let pivot = nums
                        .binary_search_by({
                            let k = target / 3;

                            move |x| {
                                if *x <= k {
                                    Ordering::Less
                                } else {
                                    Ordering::Greater
                                }
                            }
                        })
                        .unwrap_err();

                    for j in 0..pivot.min(nums.len() - 2) {
                        let second_value = nums[j];

                        if j == 0 || second_value != nums[j - 1] {
                            // Do two sum of target `target - second_value` from `j + 1` to `nums.len()`.

                            let nums = &nums[j + 1..];
                            let target = target - second_value;

                            let mut left = 0;
                            let mut right = nums.len() - 1;

                            'two_sum_loop: loop {
                                let left_value = nums[left];
                                let right_value = nums[right];

                                match (left_value + right_value).cmp(&target) {
                                    Ordering::Less => {
                                        left += 1;
                                    }
                                    Ordering::Equal => {
                                        result.push(vec![first_value, second_value, left_value, right_value]);

                                        loop {
                                            left += 1;

                                            if left == right {
                                                break 'two_sum_loop;
                                            } else if nums[left] != left_value {
                                                break;
                                            }
                                        }

                                        loop {
                                            right -= 1;

                                            if left == right {
                                                break 'two_sum_loop;
                                            } else if nums[right] != right_value {
                                                break;
                                            }
                                        }
                                    }
                                    Ordering::Greater => {
                                        right -= 1;
                                    }
                                }

                                if left >= right {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::four_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
