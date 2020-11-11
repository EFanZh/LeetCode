pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut length = nums.len();

        while length > 0 {
            let half = length / 2;
            let middle = start + half;

            if nums[middle] < target {
                start = middle + 1;
                length -= half + 1;
            } else {
                length = half;
            }
        }

        if nums.get(start).copied() == Some(target) {
            start as _
        } else {
            -1
        }
    }
}

impl super::Solution for Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
