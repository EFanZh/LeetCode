pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn find_top_2_frequent_numbers(
        nums: impl IntoIterator<Item = i32>,
        buffer: &mut HashMap<i32, u32>,
    ) -> ((i32, u32), u32) {
        for num in nums {
            buffer.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut state_1 = (0, 0);
        let mut state_2 = (0, 0);

        for state in &*buffer {
            let state = (*state.0, *state.1);

            if state.1 > state_2.1 {
                if state.1 > state_1.1 {
                    state_2 = state_1;
                    state_1 = state;
                } else {
                    state_2 = state;
                }
            }
        }

        buffer.clear();

        (state_1, state_2.1)
    }

    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut buffer = HashMap::new();
        let (even_1, even_2_count) = Self::find_top_2_frequent_numbers(nums.iter().step_by(2).copied(), &mut buffer);
        let (odd_1, odd_2_count) = Self::find_top_2_frequent_numbers(nums[1..].iter().step_by(2).copied(), &mut buffer);

        let n = nums.len() as u32;
        let odd_length = n / 2;
        let even_length = n - odd_length;

        (if even_1.0 == odd_1.0 {
            ((even_length - even_2_count) + (odd_length - odd_1.1))
                .min((even_length - even_1.1) + (odd_length - odd_2_count))
        } else {
            (even_length - even_1.1) + (odd_length - odd_1.1)
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32 {
        Self::minimum_operations(nums)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
