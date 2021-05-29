pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut states = [0_u8; 26];

        for byte in s.bytes() {
            match &mut states[usize::from(byte - b'a')] {
                state @ 0 => *state = 1,
                state @ 1 => *state = 2,
                _ => {}
            }
        }

        s.bytes()
            .position(|byte| states[usize::from(byte - b'a')] == 1)
            .map_or(-1, |i| i as _)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_uniq_char(s: String) -> i32 {
        Self::first_uniq_char(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
