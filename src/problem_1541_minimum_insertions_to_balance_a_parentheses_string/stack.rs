pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut iter = s.bytes();
        let mut stack = 0;
        let mut result = 0;

        while let Some(c) = iter.next() {
            if c == b'(' {
                stack += 1;
            } else {
                if stack == 0 {
                    result += 1;
                } else {
                    stack -= 1;
                }

                if let Some(c) = iter.next() {
                    if c == b'(' {
                        result += 1;
                        stack += 1;
                    }
                } else {
                    result += 1;

                    break;
                }
            }
        }

        result += stack * 2;

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_insertions(s: String) -> i32 {
        Self::min_insertions(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
