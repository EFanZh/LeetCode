pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut words = words;
        let word_length = words.first().map_or(0, String::len);
        let mut result = 0;

        for column in 1..word_length {
            let mut iter = words.iter().map(|word| {
                let bytes = word.as_bytes();

                u8::wrapping_sub(bytes[column], bytes[column - 1])
            });

            let first = iter.next().unwrap();

            if let Some(second) = iter.position(|c| c != first) {
                result = if second == 0 {
                    let third = iter.next().unwrap();

                    usize::from(third == first)
                } else {
                    second + 1
                };

                break;
            }
        }

        mem::take(&mut words[result])
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn odd_string(words: Vec<String>) -> String {
        Self::odd_string(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
