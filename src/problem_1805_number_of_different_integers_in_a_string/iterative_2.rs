pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
enum Integer<'a> {
    U64(u64),
    Slice(&'a [u8]),
}

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let word = word.as_bytes();
        let get = |i| word.get(i).copied();
        let mut integers = HashSet::new();
        let mut i = 0;

        'outer: while let Some(mut c) = get(i) {
            if c.is_ascii_digit() {
                if c == b'0' {
                    // Have seen '0' digit, but not '1'..='9'.

                    loop {
                        i += 1;

                        if let Some(c_2) = get(i) {
                            if c_2.is_ascii_digit() {
                                if c_2 != b'0' {
                                    c = c_2;

                                    break;
                                }
                            } else {
                                integers.insert(Integer::U64(0));

                                i += 1;

                                continue 'outer;
                            }
                        } else {
                            integers.insert(Integer::U64(0));

                            break 'outer;
                        }
                    }
                }

                let start = i;
                let mut integer = Some(u64::from(c - b'0'));

                loop {
                    i += 1;

                    if let Some(c) = get(i) {
                        if c.is_ascii_digit() {
                            integer = integer.and_then(|x| x.checked_mul(10)?.checked_add(u64::from(c - b'0')));
                        } else {
                            integers.insert(integer.map_or_else(|| Integer::Slice(&word[start..i]), Integer::U64));

                            break;
                        }
                    } else {
                        integers.insert(integer.map_or_else(|| Integer::Slice(&word[start..]), Integer::U64));

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
