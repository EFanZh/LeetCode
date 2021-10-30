pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn permute_unique_helper(nums: &mut [i32], base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if let Some((first, rest)) = nums.split_first_mut() {
            base.push(*first);

            Self::permute_unique_helper(rest, base, result);

            for i in 0..rest.len() {
                let current = &mut rest[i];

                if *current != *first {
                    mem::swap(first, current);

                    *base.last_mut().unwrap() = *first;

                    Self::permute_unique_helper(rest, base, result);
                }
            }

            // Restore order.

            for current in rest.iter_mut().rev() {
                if current != first {
                    mem::swap(first, current);
                }
            }

            base.pop();
        } else {
            result.push(base.clone());
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;

        nums.sort_unstable();

        let mut result = Vec::new();

        Self::permute_unique_helper(&mut nums, &mut Vec::new(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
