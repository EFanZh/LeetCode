pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut counts = [0_u8; 100];

        nums.iter().all(|&num| {
            let count = &mut counts[num.cast_unsigned() as usize - 1];
            let result = *count < 2;

            *count += u8::from(result);

            result
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_possible_to_split(nums: Vec<i32>) -> bool {
        Self::is_possible_to_split(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
