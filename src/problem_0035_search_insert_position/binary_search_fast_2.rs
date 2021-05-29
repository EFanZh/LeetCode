pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut length = nums.len();

        while length != 0 {
            let half = length / 2;

            if nums[start + half] < target {
                start += length - half;
            }

            length = half;
        }

        start as _
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
