pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let word = word.as_bytes();
        let get = |i| word.get(i).copied();
        let mut integers = HashSet::<&[_]>::new();
        let mut i = 0;

        'outer: while let Some(c) = get(i) {
            if c.is_ascii_digit() {
                if c == b'0' {
                    // Have seen '0' digit, but not '1'..='9'.

                    loop {
                        i += 1;

                        if let Some(c_2) = get(i) {
                            if c_2.is_ascii_digit() {
                                if c_2 != b'0' {
                                    break;
                                }
                            } else {
                                integers.insert(b"0");

                                i += 1;

                                continue 'outer;
                            }
                        } else {
                            integers.insert(b"0");

                            break 'outer;
                        }
                    }
                }

                let start = i;

                loop {
                    i += 1;

                    if let Some(c) = get(i) {
                        if !c.is_ascii_digit() {
                            integers.insert(&word[start..i]);

                            break;
                        }
                    } else {
                        integers.insert(&word[start..]);

                        break 'outer;
                    }
                }
            }

            i += 1;
        }

        integers.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_different_integers(word: String) -> i32 {
        Self::num_different_integers(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
