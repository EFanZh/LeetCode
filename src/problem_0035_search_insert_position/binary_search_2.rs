pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut length = nums.len();

        while length != 0 {
            let half = length / 2;

            if nums[start + half] < target {
                start += half + 1;
                length -= half + 1;
            } else {
                length = half;
            }
        }

        start as _
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
