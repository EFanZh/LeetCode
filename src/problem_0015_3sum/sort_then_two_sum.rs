pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
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

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        // Must have at least 3 elements.

        if nums.len() > 2 {
            let (nums, pivot) = {
                let mut temp = nums;
                let (left, right) = Self::partition(temp.as_mut(), |x| *x <= 0);
                let pivot = left.len();

                left.sort_unstable();
                right.sort_unstable();

                (temp, pivot)
            };

            for i in 0..pivot.min(nums.len() - 2) {
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

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::three_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
