pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
        let mut states = HashMap::from([((0, 0), 0)]);
        let mut state = (0, 0);
        let mut result = 0;

        (1..).zip(nums).for_each(|(i, num)| {
            state.0 ^= num;
            state.1 += 1 - (num & 1) * 2;
            states.entry(state).or_insert(i);

            if let Some(prev) = states.get(&state) {
                result = u32::max(result, i - prev);
            }
        });

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
        Self::max_balanced_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
