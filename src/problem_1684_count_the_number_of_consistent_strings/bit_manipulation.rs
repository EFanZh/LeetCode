pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn as_bit(c: u8) -> u32 {
        1 << (c - b'a')
    }

    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_bits = 0_u32;

        for c in allowed.into_bytes() {
            allowed_bits |= Self::as_bit(c);
        }

        words
            .into_iter()
            .filter(|word| word.bytes().all(|c| allowed_bits & Self::as_bit(c) != 0))
            .count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        Self::count_consistent_strings(allowed, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
