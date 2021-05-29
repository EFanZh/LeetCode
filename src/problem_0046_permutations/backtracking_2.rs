pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn permute_helper(nums: &mut [i32], split: usize, result: &mut Vec<Vec<i32>>) {
        if split == nums.len() {
            result.push(nums.to_vec());
        } else {
            Self::permute_helper(nums, split + 1, result);

            for i in split + 1..nums.len() {
                nums.swap(split, i);

                Self::permute_helper(nums, split + 1, result);

                nums.swap(split, i);
            }
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::permute_helper(&mut nums, 0, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
