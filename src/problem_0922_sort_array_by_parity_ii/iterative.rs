pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0;
        let mut j = 1;

        while let Some(left) = nums.get(i) {
            if left % 2 != 0 {
                while nums[j] % 2 != 0 {
                    j += 2;
                }

                nums.swap(i, j);

                j += 2;
            }

            i += 2;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array_by_parity_ii(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
