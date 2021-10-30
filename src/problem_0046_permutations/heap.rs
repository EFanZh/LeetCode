pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn permute_helper_even(nums: &mut [i32], k: usize, result: &mut Vec<Vec<i32>>) {
        if k == 0 {
            result.push(nums.to_vec());
        } else {
            Self::permute_helper_odd(nums, k - 1, result);

            for i in 0..k - 1 {
                nums.swap(i, k - 1);

                Self::permute_helper_odd(nums, k - 1, result);
            }
        }
    }

    fn permute_helper_odd(nums: &mut [i32], k: usize, result: &mut Vec<Vec<i32>>) {
        Self::permute_helper_even(nums, k - 1, result);

        for _ in 0..k - 1 {
            nums.swap(0, k - 1);

            Self::permute_helper_even(nums, k - 1, result);
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut nums = nums;
        let nums_len = nums.len();

        if nums.len() % 2 == 0 {
            Self::permute_helper_even(&mut nums, nums_len, &mut result);
        } else {
            Self::permute_helper_odd(&mut nums, nums_len, &mut result);
        }

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
