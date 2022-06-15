pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_chars(s: &str) -> [u16; 26] {
        let mut result = [0; 26];

        for c in s.bytes() {
            result[usize::from(c) - usize::from(b'a')] += 1;
        }

        result
    }

    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let chars = Self::count_chars(&chars);
        let mut result = 0;

        'outer: for word in &words {
            let mut available = chars;

            for c in word.bytes() {
                let count = &mut available[usize::from(c) - usize::from(b'a')];

                if *count == 0 {
                    continue 'outer;
                }

                *count -= 1;
            }

            result += word.len();
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_characters(words: Vec<String>, chars: String) -> i32 {
        Self::count_characters(words, chars)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
