pub struct Solution;

impl Solution {
    fn is_palindrome(s: &[u8]) -> bool {
        let mut iter = s.iter();

        while let (Some(left), Some(right)) = (iter.next(), iter.next_back()) {
            if left != right {
                return false;
            }
        }

        true
    }

    pub fn shortest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let split = (0..=s.len()).rev().find(|i| Self::is_palindrome(&s[..*i])).unwrap();
        let mut result = s[split..].iter().copied().rev().collect::<Vec<_>>();

        result.extend_from_slice(s);

        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn shortest_palindrome(s: String) -> String {
        Self::shortest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
