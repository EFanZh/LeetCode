pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut result = Vec::new();

        for (i, word) in s.split(' ').enumerate() {
            result.extend(iter::repeat_with(String::new).take(word.len().saturating_sub(result.len())));

            for (row, c) in result.iter_mut().zip(word.bytes()) {
                row.extend(iter::repeat(' ').take(i.saturating_sub(row.len())));
                row.push(char::from(c));
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn print_vertically(s: String) -> Vec<String> {
        Self::print_vertically(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
