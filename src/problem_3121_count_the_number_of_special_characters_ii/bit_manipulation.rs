pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let state = word.into_bytes().into_iter().fold(0_u64, |state, c| {
            let shift = (c & 31) * 2;
            let transition = ((0b_0110_0110_u64 << 32) | 0b_1111_0101_u64) >> (c & 32);
            let new_state = transition >> (((state >> shift) & 0b_11) * 2);
            let mask = 0b_11 << shift;

            (state & !mask) | ((new_state << shift) & mask)
        });

        (state & (state >> 1) & 0x_5555_5555_5555_5555).count_ones() as _
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
