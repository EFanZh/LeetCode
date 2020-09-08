pub struct Solution;

impl Solution {
    fn rob_helper(nums: &[i32]) -> i32 {
        let mut prev = 0;
        let mut current = 0;

        for num in nums {
            let new_current = current.max(prev + num);

            prev = current;
            current = new_current;
        }

        current
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            _ => (nums[0] + Self::rob_helper(&nums[2..nums.len() - 1])).max(Self::rob_helper(&nums[1..])),
        }
    }
}

impl super::Solution for Solution {
    fn rob(nums: Vec<i32>) -> i32 {
        Self::rob(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
