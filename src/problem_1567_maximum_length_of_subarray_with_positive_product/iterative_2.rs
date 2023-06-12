pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut result = 0_u32;
        let mut state = (0, 0); // (max_negative_length, max_positive_length).

        for num in nums {
            if num == 0 {
                state = (0, 0);
            } else {
                state.0 += u32::from(state.0 != 0);
                state.1 += 1;

                if num < 0 {
                    state = (state.1, state.0);
                }

                result = result.max(state.1);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_max_len(nums: Vec<i32>) -> i32 {
        Self::get_max_len(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
