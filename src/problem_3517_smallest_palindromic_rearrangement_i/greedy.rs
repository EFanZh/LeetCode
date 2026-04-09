pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut s = s.into_bytes();
        let mut counts = [0; 26];

        for &c in &s[..s.len() / 2] {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut left_cursor = 0;
        let mut right_cursor = s.len();

        (b'a'..).zip(&counts).for_each(|(c, &count)| {
            let new_left_cursor = left_cursor + count;
            let new_right_cursor = right_cursor - count;

            s[left_cursor..new_left_cursor].fill(c);
            s[new_right_cursor..right_cursor].fill(c);

            left_cursor = new_left_cursor;
            right_cursor = new_right_cursor;
        });

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_palindrome(s: String) -> String {
        Self::smallest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
