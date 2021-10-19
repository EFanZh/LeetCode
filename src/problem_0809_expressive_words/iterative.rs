pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;
use std::mem;

impl Solution {
    fn iter_groups(s: &[u8]) -> impl Iterator<Item = (u8, u8)> + '_ {
        let mut base_iter = s.iter();
        let mut prev = *base_iter.next().unwrap();
        let mut length = 1;

        iter::from_fn(move || {
            if length == 0 {
                None
            } else {
                loop {
                    if let Some(&c) = base_iter.next() {
                        if c == prev {
                            length += 1;
                        } else {
                            return Some((mem::replace(&mut prev, c), mem::replace(&mut length, 1)));
                        }
                    } else {
                        return Some((prev, mem::replace(&mut length, 0)));
                    }
                }
            }
        })
    }

    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s_groups = Self::iter_groups(s.as_bytes()).collect::<Vec<_>>();
        let mut result = 0;

        for word in &words {
            let mut base_iter = s_groups.iter();
            let mut word_iter = Self::iter_groups(word.as_bytes());

            loop {
                match (base_iter.next(), word_iter.next()) {
                    (None, None) => {
                        result += 1;

                        break;
                    }
                    (None, Some(_)) | (Some(_), None) => break,
                    (Some(&(base_letter, base_count)), Some((word_letter, word_count))) => {
                        if base_letter != word_letter {
                            break;
                        }

                        if base_count < 3 {
                            if base_count != word_count {
                                break;
                            }
                        } else if base_count < word_count {
                            break;
                        }
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn expressive_words(s: String, words: Vec<String>) -> i32 {
        Self::expressive_words(s, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
