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

fn lower_bound<T, F: FnMut(&T) -> bool>(values: &[T], mut f: F) -> usize {
    let mut size = values.len();

    if size == 0 {
        return 0;
    }

    let mut left = 0;

    while size > 1 {
        let half = size / 2;
        let middle = left + half;

        if f(&values[middle]) {
            left = middle;
        }

        size -= half;
    }

    if f(&values[left]) {
        left + 1
    } else {
        left
    }
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

                    let pivot = lower_bound(nums, {
                        let k = target / 3;

                        move |x| *x <= k
                    });

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
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
