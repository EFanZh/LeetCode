pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words[left as u32 as usize..=right as u32 as usize]
            .iter()
            .filter(|word| {
                if let [b'a' | b'e' | b'i' | b'o' | b'u', rest @ ..] = word.as_bytes() {
                    matches!(rest, [] | [.., b'a' | b'e' | b'i' | b'o' | b'u'])
                } else {
                    false
                }
            })
            .count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        Self::vowel_strings(words, left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
