pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let text = text.into_bytes();
        let n = text.len();
        let mut results = HashSet::new();

        for offset in 1..=n / 2 {
            let mut length = 0;

            for ((top, bottom), window) in text.iter().zip(&text[offset..]).zip(text.windows(offset)) {
                if top == bottom {
                    length += 1;

                    if length >= offset {
                        results.insert(window);
                    }
                } else {
                    length = 0;
                }
            }
        }

        results.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distinct_echo_substrings(text: String) -> i32 {
        Self::distinct_echo_substrings(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
