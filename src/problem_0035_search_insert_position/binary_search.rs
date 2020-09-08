pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut size = nums.len();

        while size > 0 {
            let half = size / 2;
            let middle = left + half;

            if nums[middle] < target {
                left = middle + 1;
                size -= half + 1;
            } else {
                size = half;
            }
        }

        left as _
    }
}

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
