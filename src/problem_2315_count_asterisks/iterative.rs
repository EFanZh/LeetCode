pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut result = 0;
        let mut iter = s.bytes();

        loop {
            loop {
                if let Some(c) = iter.next() {
                    if c == b'|' {
                        break;
                    }

                    result += i32::from(c == b'*');
                } else {
                    return result;
                }
            }

            iter.find(|&c| c == b'|');
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_asterisks(s: String) -> i32 {
        Self::count_asterisks(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
