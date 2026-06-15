pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_vowels(s: &[u8]) -> u32 {
        s.iter()
            .filter(|&&c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
            .count() as _
    }

    pub fn reverse_words(s: String) -> String {
        let mut s = s.into_bytes();
        let mut iter = s.split_mut(|&c| c == b' ');
        let vowels = Self::count_vowels(iter.next().unwrap());

        iter.for_each(|word| {
            if Self::count_vowels(word) == vowels {
                word.reverse();
            }
        });

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_words(s: String) -> String {
        Self::reverse_words(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
