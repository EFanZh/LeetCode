pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();

        if size == 0 {
            return 0;
        }

        let mut left = 0;

        while size > 1 {
            let half = size / 2;
            let middle = left + half;

            if nums[middle] < target {
                left = middle;
            }

            size -= half;
        }

        (if nums[left] < target { left + 1 } else { left }) as _
    }
}

impl super::Solution for Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search_insert(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
