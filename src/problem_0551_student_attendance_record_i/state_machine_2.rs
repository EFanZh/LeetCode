pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut iter = s.bytes();

        loop {
            // A0L0.

            match iter.next() {
                None => return true,
                Some(b'A') => break,
                Some(b'L') => {}
                Some(_) => continue,
            }

            // A0L1.

            match iter.next() {
                None => return true,
                Some(b'A') => break,
                Some(b'L') => {}
                Some(_) => continue,
            }

            // A0L2.

            match iter.next() {
                None => return true,
                Some(b'A') => break,
                Some(b'L') => return false,
                Some(_) => continue,
            }
        }

        loop {
            // A1L0.

            match iter.next() {
                None => return true,
                Some(b'A') => return false,
                Some(b'L') => {}
                Some(_) => continue,
            }

            // A1L1.

            match iter.next() {
                None => return true,
                Some(b'A') => return false,
                Some(b'L') => {}
                Some(_) => continue,
            }

            // A1L2.

            match iter.next() {
                None => return true,
                Some(b'A') | Some(b'L') => return false,
                Some(_) => continue,
            }
        }
    }
}

impl super::Solution for Solution {
    fn check_record(s: String) -> bool {
        Self::check_record(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
