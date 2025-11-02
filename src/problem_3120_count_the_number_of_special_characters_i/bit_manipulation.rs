pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut states = [0_u32; 2];

        for c in word.into_bytes() {
            states[usize::from(c >> 5) & 1] |= 1 << (c & 31);
        }

        let [upper_cases, lower_cases] = states;

        (upper_cases & lower_cases).count_ones().cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_special_chars(word: String) -> i32 {
        Self::number_of_special_chars(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
