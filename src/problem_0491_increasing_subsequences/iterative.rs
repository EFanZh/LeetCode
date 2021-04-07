pub struct Solution;

use std::cmp::Ordering;

// https://leetcode.com/problems/increasing-subsequences/discuss/97147/Java-solution-beats-100/363291.

impl Solution {
    fn helper_2(base: &mut Vec<i32>, prev: i32, nums: &[i32], result: &mut Vec<Vec<i32>>) {
        if let Some((&num, rest)) = nums.split_first() {
            match num.cmp(&prev) {
                Ordering::Less => {
                    Self::helper_2(base, prev, rest, result);
                }
                Ordering::Equal => {
                    base.push(num);
                    Self::helper_2(base, num, rest, result);
                    base.pop();
                }
                Ordering::Greater => {
                    base.push(num);
                    Self::helper_2(base, num, rest, result);
                    base.pop();

                    Self::helper_2(base, prev, rest, result);
                }
            }
        } else {
            result.push(base.clone());
        }
    }

    fn helper_1(base: &mut Vec<i32>, prev: i32, nums: &[i32], result: &mut Vec<Vec<i32>>) {
        if let Some((&num, rest)) = nums.split_first() {
            match num.cmp(&prev) {
                Ordering::Less => {
                    Self::helper_1(base, prev, rest, result);
                }
                Ordering::Equal => {
                    base.push(num);
                    Self::helper_2(base, num, rest, result);
                    base.pop();
                }
                Ordering::Greater => {
                    base.push(num);
                    Self::helper_2(base, num, rest, result);
                    base.pop();

                    Self::helper_1(base, prev, rest, result);
                }
            }
        }
    }

    fn helper_0(base: &mut Vec<i32>, nums: &[i32], result: &mut Vec<Vec<i32>>) {
        if let Some((&num, rest)) = nums.split_first() {
            base.push(num);
            Self::helper_1(base, num, rest, result);
            base.pop();

            Self::helper_0(base, rest, result);
        }
    }

    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::helper_0(&mut Vec::new(), &nums, &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::find_subsequences(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
