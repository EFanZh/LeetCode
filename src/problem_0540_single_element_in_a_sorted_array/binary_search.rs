pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left != right {
            let middle = ((left + right) / 2) & !1;

            if nums[middle] == nums[middle + 1] {
                left = middle + 2;
            } else {
                right = middle;
            }
        }

        nums[left]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        Self::single_non_duplicate(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
