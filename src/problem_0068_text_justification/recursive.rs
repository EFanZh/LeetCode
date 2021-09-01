pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;
use std::mem;

impl Solution {
    fn full_justify_helper(mut line: String, words: &mut [String], max_width: usize, result: &mut Vec<String>) {
        let mut min_line_length = line.len();

        line.reserve(max_width - min_line_length);

        for (i, word) in words.iter_mut().enumerate() {
            let new_min_line_length = min_line_length + 1 + word.len();

            if new_min_line_length <= max_width {
                min_line_length = new_min_line_length;
            } else {
                let new_line = mem::take(word);

                if i == 0 {
                    line.extend(iter::repeat(' ').take(max_width - min_line_length));
                } else {
                    let spaces = max_width - min_line_length + i;
                    let narrow_gap = spaces / i;
                    let wide_gap = narrow_gap + 1;
                    let (words_1, words_2) = words[..i].split_at(spaces % i);

                    for word in words_1 {
                        line.extend(iter::repeat(' ').take(wide_gap));
                        line.push_str(word);
                    }

                    for word in words_2 {
                        line.extend(iter::repeat(' ').take(narrow_gap));
                        line.push_str(word);
                    }
                }

                result.push(line);

                return Self::full_justify_helper(new_line, &mut words[i + 1..], max_width, result);
            }
        }

        for word in words {
            line.push(' ');
            line.push_str(word);
        }

        line.extend(iter::repeat(' ').take(max_width - line.len()));

        result.push(line);
    }

    pub fn full_justify(mut words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut result = Vec::new();
        let (first_word, rest_words) = words.split_first_mut().unwrap();

        Self::full_justify_helper(mem::take(first_word), rest_words, max_width as _, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        Self::full_justify(words, max_width)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
