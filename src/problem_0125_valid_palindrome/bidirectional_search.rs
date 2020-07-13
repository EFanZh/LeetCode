pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut iter = s.into_bytes().into_iter().filter(u8::is_ascii_alphanumeric);

        while let Some(first) = iter.next() {
            if let Some(last) = iter.next_back() {
                if !first.eq_ignore_ascii_case(&last) {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn is_palindrome(s: String) -> bool {
        Self::is_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
