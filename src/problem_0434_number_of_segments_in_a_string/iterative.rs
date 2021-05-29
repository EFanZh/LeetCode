pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut result = 0;
        let mut iter = s.bytes();

        while let Some(c) = iter.next() {
            if c != b' ' {
                result += 1;

                loop {
                    if let Some(c) = iter.next() {
                        if c == b' ' {
                            break;
                        }
                    } else {
                        return result;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_segments(s: String) -> i32 {
        Self::count_segments(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
