pub struct Solution;

use std::mem;

impl Solution {
    fn permute_helper(nums: &mut [i32], base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if let Some((first, rest)) = nums.split_first_mut() {
            base.push(*first);

            Self::permute_helper(rest, base, result);

            for i in 0..rest.len() {
                mem::swap(first, &mut rest[i]);

                *base.last_mut().unwrap() = *first;
                Self::permute_helper(rest, base, result);

                mem::swap(first, &mut rest[i]);
            }

            base.pop();
        } else {
            result.push(base.clone());
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let nums_len = nums.len();

        Self::permute_helper(&mut nums, &mut Vec::with_capacity(nums_len), &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
