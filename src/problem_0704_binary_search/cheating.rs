pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).map(|i| i as _).unwrap_or(-1)
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
