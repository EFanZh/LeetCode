pub struct Solution {}

use std::convert::TryFrom;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;

        while let Some(mut num) = nums.get(i).copied() {
            while let Ok(expected_index) = usize::try_from(num - 1) {
                if expected_index != i {
                    if let Some(next_num) = nums.get(expected_index).copied() {
                        if usize::try_from(next_num - 1) != Ok(expected_index) {
                            nums.swap(i, expected_index);
                            num = next_num;

                            continue;
                        }
                    }
                }

                break;
            }

            i += 1;
        }

        for (item, expected) in nums.iter().zip(1..) {
            if *item != expected {
                return expected;
            }
        }

        nums.len() as i32 + 1
    }
}

impl super::Solution for Solution {
    fn first_missing_positive(nums: Vec<i32>) -> i32 {
        Self::first_missing_positive(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
