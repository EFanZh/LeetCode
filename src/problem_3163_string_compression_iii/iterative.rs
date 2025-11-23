pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut result = String::new();
        let mut iter = word.bytes();

        'outer: while let Some(mut first) = iter.next() {
            let mut count = b'1';

            loop {
                if let Some(c) = iter.next() {
                    if c == first {
                        if count < b'8' {
                            count += 1;
                        } else {
                            result.extend(['9', char::from(first)]);

                            break;
                        }
                    } else {
                        result.extend([char::from(count), char::from(first)]);
                        first = c;
                        count = b'1';
                    }
                } else {
                    result.extend([char::from(count), char::from(first)]);

                    break 'outer;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn compressed_string(word: String) -> String {
        Self::compressed_string(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
