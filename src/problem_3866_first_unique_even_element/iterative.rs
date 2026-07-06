pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn first_unique_even(nums: Vec<i32>) -> i32 {
        let mut states = [0_u8; 100];

        for &num in &nums {
            if num & 1 == 0 {
                let state = &mut states[num.cast_unsigned() as usize - 1];

                *state = if *state == 0 { 1 } else { 2 };
            }
        }

        nums.iter()
            .copied()
            .find(|&num| states[num.cast_unsigned() as usize - 1] == 1)
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_unique_even(nums: Vec<i32>) -> i32 {
        Self::first_unique_even(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
