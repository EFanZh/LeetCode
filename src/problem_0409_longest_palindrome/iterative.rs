pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        const COUNTS_LENGTH: usize = (b'z' - b'A' + 1) as _;
        let mut counts: [i32; COUNTS_LENGTH] = [0; COUNTS_LENGTH];

        for c in s.bytes() {
            counts[usize::from(c - b'A')] += 1;
        }

        let mut result = 0;

        for c in (b'A'..=b'Z').chain(b'a'..=b'z') {
            result += counts[usize::from(c - b'A')] & !1;
        }

        if result as usize == s.len() {
            result
        } else {
            result + 1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_palindrome(s: String) -> i32 {
        Self::longest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
