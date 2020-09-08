pub struct Solution;

use std::iter;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut result = Vec::new();
        let mut word_iter = words.iter().map(String::as_str).enumerate();

        if let Some((mut i, mut first_word)) = word_iter.next() {
            let mut line = first_word.to_string();
            let mut min_line_length = line.len();

            loop {
                if let Some((j, word)) = word_iter.next() {
                    let new_min_line_length = min_line_length + 1 + word.len();

                    if new_min_line_length <= max_width {
                        min_line_length = new_min_line_length;
                    } else {
                        let gaps = j - i - 1;

                        if gaps == 0 {
                            line.extend(iter::repeat(' ').take(max_width - line.len()));
                        } else {
                            let spaces = max_width - min_line_length + gaps;
                            let narrow_gap = spaces / gaps;
                            let wide_gap = narrow_gap + 1;
                            let (words_1, words_2) = words[i + 1..j].split_at(spaces % gaps);

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

                        i = j;
                        first_word = word;
                        line = first_word.to_string();
                        min_line_length = line.len();
                    }
                } else {
                    for word in &words[i + 1..] {
                        line.push(' ');
                        line.push_str(word);
                    }

                    line.extend(iter::repeat(' ').take(max_width - line.len()));

                    result.push(line);

                    break;
                }
            }
        }

        result
    }
}

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
