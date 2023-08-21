pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let limit = u64::from(limit as u32);
        let sum = nums.into_iter().map(i64::from).sum::<i64>();
        let diff = (i64::from(goal) - sum).unsigned_abs();

        ((diff + limit - 1) / limit) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        Self::min_elements(nums, limit, goal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
