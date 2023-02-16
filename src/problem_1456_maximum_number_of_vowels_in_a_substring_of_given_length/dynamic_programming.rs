pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_vowel(c: u8) -> bool {
        matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let (left, right) = s.as_bytes().split_at(k as _);
        let mut count = left.iter().filter(|&&c| Self::is_vowel(c)).count();
        let mut result = count;

        for (old, &new) in s.bytes().zip(right) {
            count -= usize::from(Self::is_vowel(old));
            count += usize::from(Self::is_vowel(new));
            result = result.max(count);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_vowels(s: String, k: i32) -> i32 {
        Self::max_vowels(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
