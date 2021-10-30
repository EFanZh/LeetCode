use std::mem;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn permute_unique_helper(nums: &mut [i32], split: usize, result: &mut Vec<Vec<i32>>) {
        if split == nums.len() {
            result.push(nums.to_vec());
        } else {
            Self::permute_unique_helper(nums, split + 1, result);

            for i in split + 1..nums.len() {
                if nums[i] != nums[split] {
                    nums.swap(split, i);

                    Self::permute_unique_helper(nums, split + 1, result);
                }
            }

            // Restore order.

            let (first, rest) = nums[split..].split_first_mut().unwrap();

            for current in rest.iter_mut().rev() {
                if current != first {
                    mem::swap(current, first);
                }
            }
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;

        nums.sort_unstable();

        let mut result = Vec::new();

        Self::permute_unique_helper(&mut nums, 0, &mut result);

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
