pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let n = pattern.len();
        let mut result = String::with_capacity(n + 1);
        let mut length = 0;
        let mut c = b'1';
        let mut iter = pattern.bytes();

        'outer: loop {
            if let Some(direction) = iter.next() {
                if direction == b'I' {
                    length += 1;
                    c += 1;
                } else {
                    result.extend((c - length..c).map(char::from));

                    length = 2;
                    c += 2;

                    loop {
                        if let Some(direction) = iter.next() {
                            if direction == b'I' {
                                break;
                            }

                            length += 1;
                            c += 1;
                        } else {
                            result.extend((c - length..c).map(char::from).rev());

                            break 'outer;
                        }
                    }

                    result.extend((c - length..c).map(char::from).rev());
                    length = 0;
                }
            } else {
                result.extend((c - length..=c).map(char::from));

                break;
            }
        }

        drop(pattern);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_number(pattern: String) -> String {
        Self::smallest_number(pattern)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
