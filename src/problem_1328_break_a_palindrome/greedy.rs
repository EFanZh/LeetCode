pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut bytes = palindrome.into_bytes();
        let n = bytes.len();

        #[allow(clippy::option_if_let_else)] // False positive.
        if let Some(c) = bytes[..n / 2].iter_mut().find(|&&mut c| c != b'a') {
            *c = b'a';
        } else if n > 1 {
            *bytes.last_mut().unwrap() = b'b';
        } else {
            bytes.clear();
        }

        String::from_utf8(bytes).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn break_palindrome(palindrome: String) -> String {
        Self::break_palindrome(palindrome)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
