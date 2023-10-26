pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut state = (0, 0);

        for num in nums {
            let num = i64::from(num);

            state = (state.0.max(state.1 + num), state.1.max(state.0 - num));
        }

        state.0.max(state.1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        Self::max_alternating_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
