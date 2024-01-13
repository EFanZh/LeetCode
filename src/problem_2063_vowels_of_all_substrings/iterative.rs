pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let mut result = 0;
        let total = word.len() + 1;

        for (i, c) in (1..).zip(word.bytes()) {
            if matches!(c, b'a' | b'e' | b'i' | b'o' | b'u') {
                result += i * (total - i);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_vowels(word: String) -> i64 {
        Self::count_vowels(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
