pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut iter = target.bytes();
        let mut result = 0;

        'outer: while let Some(c) = iter.next() {
            if c != b'0' {
                result += 1;

                loop {
                    if let Some(c) = iter.next() {
                        if c == b'0' {
                            result += 1;

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips(target: String) -> i32 {
        Self::min_flips(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
