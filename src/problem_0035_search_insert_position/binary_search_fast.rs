pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let mut length = nums.len();
            let mut start = 0;

            while length != 1 {
                let half = length / 2;
                let middle = start + half;

                if nums[middle] < target {
                    start = middle;
                }

                length -= half;
            }

            (if nums[start] < target { start + 1 } else { start }) as _
        }
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
