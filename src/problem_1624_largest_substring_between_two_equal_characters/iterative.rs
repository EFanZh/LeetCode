pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut first_positions = [0_u16; 26];
        let mut i = 0;
        let mut result_plus_1 = 0;

        for c in s.into_bytes() {
            i += 1;

            let first = &mut first_positions[usize::from(c) - usize::from(b'a')];

            if *first == 0 {
                *first = i;
            } else {
                result_plus_1 = result_plus_1.max(i - *first);
            }
        }

        i32::from(result_plus_1 as i16 - 1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_length_between_equal_characters(s: String) -> i32 {
        Self::max_length_between_equal_characters(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
