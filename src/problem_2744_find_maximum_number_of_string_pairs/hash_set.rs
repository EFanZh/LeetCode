pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut reversed = HashSet::new();
        let mut result = 0;

        for word in words {
            let mut word = word.into_bytes();

            if reversed.remove(word.as_slice()) {
                result += 1;
            } else {
                word.reverse();
                reversed.insert(word);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        Self::maximum_number_of_string_pairs(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
