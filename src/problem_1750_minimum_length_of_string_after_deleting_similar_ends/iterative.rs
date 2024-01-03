pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut iter = s.as_bytes().iter();
        let mut first = iter.next().copied().unwrap();

        if let Some(mut last) = iter.next_back().copied() {
            loop {
                if first == last {
                    // Consume left side that has the same value as `first`.

                    loop {
                        if let Some(&left) = iter.next() {
                            if left != first {
                                first = left;

                                break;
                            }
                        } else {
                            return 0;
                        }
                    }

                    // Consume right side that has the same value as `last`.

                    loop {
                        if let Some(&right) = iter.next_back() {
                            if right != last {
                                last = right;

                                break;
                            }
                        } else {
                            return 1;
                        }
                    }
                } else {
                    return iter.len() as i32 + 2;
                }
            }
        } else {
            1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_length(s: String) -> i32 {
        Self::minimum_length(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
