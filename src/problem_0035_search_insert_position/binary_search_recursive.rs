pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn search_insert_helper(nums: &[i32], target: i32, offset: usize) -> usize {
        if nums.is_empty() {
            offset
        } else {
            let middle = nums.len() / 2;

            if nums[middle] < target {
                Self::search_insert_helper(&nums[middle + 1..], target, offset + middle + 1)
            } else {
                Self::search_insert_helper(&nums[..middle], target, offset)
            }
        }
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_insert_helper(&nums, target, 0) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_insert(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
