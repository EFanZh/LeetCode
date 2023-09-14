pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut result = 0;
        let mut iter = word.bytes();

        'outer: while let Some(c) = iter.next() {
            if c != b'a' {
                continue;
            }

            // Start.

            'start: loop {
                let mut length = 1;

                // a.

                loop {
                    match iter.next() {
                        None => break 'outer,
                        Some(b'a') => length += 1,
                        Some(b'e') => break,
                        Some(_) => continue 'outer,
                    }
                }

                length += 1;

                // e.

                loop {
                    match iter.next() {
                        None => break 'outer,
                        Some(b'a') => continue 'start,
                        Some(b'e') => length += 1,
                        Some(b'i') => break,
                        Some(_) => continue 'outer,
                    }
                }

                length += 1;

                // i.

                loop {
                    match iter.next() {
                        None => break 'outer,
                        Some(b'a') => continue 'start,
                        Some(b'i') => length += 1,
                        Some(b'o') => break,
                        Some(_) => continue 'outer,
                    }
                }

                length += 1;

                // o.

                loop {
                    match iter.next() {
                        None => break 'outer,
                        Some(b'a') => continue 'start,
                        Some(b'o') => length += 1,
                        Some(b'u') => break,
                        Some(_) => continue 'outer,
                    }
                }

                length += 1;

                // u.

                loop {
                    match iter.next() {
                        None => {
                            result = result.max(length);

                            break 'outer;
                        }
                        Some(b'a') => {
                            result = result.max(length);

                            continue 'start;
                        }
                        Some(b'u') => length += 1,
                        Some(_) => {
                            result = result.max(length);

                            continue 'outer;
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
    fn longest_beautiful_substring(word: String) -> i32 {
        Self::longest_beautiful_substring(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
