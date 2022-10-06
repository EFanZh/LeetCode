pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn last_substring(palindrome: String) -> String {
        let s = palindrome.as_bytes();
        let mut best_start = 0;
        let mut start = 1;
        let mut length = 0;

        while let Some(&c) = s.get(start + length) {
            match c.cmp(&s[best_start + length]) {
                Ordering::Less => {
                    start += length + 1;
                    length = 0;
                }
                Ordering::Equal => length += 1,
                Ordering::Greater => {
                    best_start = start.max(best_start + length + 1);
                    start = best_start + 1;
                    length = 0;
                }
            }
        }

        let mut result = palindrome;

        result.replace_range(..best_start, "");

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn last_substring(s: String) -> String {
        Self::last_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
