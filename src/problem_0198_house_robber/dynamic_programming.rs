pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max_if_skip = 0;
        let mut max_if_rob = 0;

        for num in nums {
            let new_max_if_skip = max_if_skip.max(max_if_rob);
            let new_max_if_rob = max_if_skip + num;

            max_if_skip = new_max_if_skip;
            max_if_rob = new_max_if_rob;
        }

        max_if_skip.max(max_if_rob)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
