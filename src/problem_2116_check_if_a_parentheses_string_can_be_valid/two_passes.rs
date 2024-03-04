pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        s.len() % 2 == 0 && {
            let iter = s.bytes().zip(locked.bytes());

            // Forward.

            let mut budget = 0;
            let mut stack = 0;

            for (c, locked) in iter.clone() {
                if locked == b'0' {
                    budget += 1;
                } else if c == b'(' {
                    stack += 1;
                } else if stack == 0 {
                    if budget == 0 {
                        return false;
                    }

                    budget -= 1;
                } else {
                    stack -= 1;
                }
            }

            if budget < stack {
                return false;
            }

            // Backward.

            budget = 0;
            stack = 0;

            for (c, locked) in iter.rev() {
                if locked == b'0' {
                    budget += 1;
                } else if c == b'(' {
                    if stack == 0 {
                        if budget == 0 {
                            return false;
                        }

                        budget -= 1;
                    } else {
                        stack -= 1;
                    }
                } else {
                    stack += 1;
                }
            }

            stack <= budget
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_be_valid(s: String, locked: String) -> bool {
        Self::can_be_valid(s, locked)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
