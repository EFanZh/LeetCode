pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut vowels = 0_u32;
        let mut consonants = 0_u32;

        for c in s.into_bytes() {
            if c.is_ascii_lowercase() {
                if matches!(c, b'a' | b'e' | b'i' | b'o' | b'u') {
                    vowels += 1;
                } else {
                    consonants += 1;
                }
            }
        }

        vowels.checked_div(consonants).unwrap_or(0).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn vowel_consonant_score(s: String) -> i32 {
        Self::vowel_consonant_score(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
