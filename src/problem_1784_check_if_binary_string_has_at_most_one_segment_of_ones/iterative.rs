pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut iter = s.bytes();

        loop {
            if let Some(c) = iter.next() {
                if c == b'0' {
                    break;
                }
            } else {
                return true;
            }
        }

        iter.all(|c| c == b'0')
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_ones_segment(s: String) -> bool {
        Self::check_ones_segment(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
