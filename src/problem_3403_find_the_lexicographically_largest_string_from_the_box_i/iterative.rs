pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        let mut word = word;
        let num_friends = num_friends.cast_unsigned() as usize;

        if num_friends > 1 {
            let bytes = word.as_bytes();
            let max_length = bytes.len() - (num_friends - 1);
            let mut best_start = 0;
            let mut start = 1;
            let mut length = 0;

            while let Some(&c) = bytes.get(start + length) {
                match c.cmp(&bytes[best_start + length]) {
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

            word.truncate(best_start + max_length);
            word.replace_range(..best_start, "");
        }

        word
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn answer_string(word: String, num_friends: i32) -> String {
        Self::answer_string(word, num_friends)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
