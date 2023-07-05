pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_vowel(c: u8) -> bool {
        matches!(c, b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u')
    }

    pub fn halves_are_alike(s: String) -> bool {
        let (left, right) = s.as_bytes().split_at(s.len() / 2);
        let mut vowels = 0_u16;

        for &c in left {
            vowels += u16::from(Self::is_vowel(c));
        }

        for &c in right {
            vowels = vowels.wrapping_sub(u16::from(Self::is_vowel(c)));

            if vowels == u16::MAX {
                return false;
            }
        }

        vowels == 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn halves_are_alike(s: String) -> bool {
        Self::halves_are_alike(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
