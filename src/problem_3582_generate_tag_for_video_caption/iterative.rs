pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut result = caption.into_bytes();
        let mut i = 0;
        let mut retained = 0;
        let mut is_first_word = true;

        'outer: while let Some(&c) = result.get(i) {
            i += 1;

            if c != b' ' {
                result[retained] = if is_first_word {
                    is_first_word = false;

                    c.to_ascii_lowercase()
                } else {
                    c.to_ascii_uppercase()
                };

                retained += 1;

                if retained == 99 {
                    break;
                }

                loop {
                    if let Some(&c) = result.get(i) {
                        i += 1;

                        if c == b' ' {
                            break;
                        }

                        result[retained] = c.to_ascii_lowercase();
                        retained += 1;

                        if retained == 99 {
                            break 'outer;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
        }

        result.truncate(retained);
        result.insert(0, b'#');

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn generate_tag(caption: String) -> String {
        Self::generate_tag(caption)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
