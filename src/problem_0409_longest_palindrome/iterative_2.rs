pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        const COUNTS_LENGTH: usize = (b'z' - b'A' + 1) as _;
        let mut counts: [bool; COUNTS_LENGTH] = [false; COUNTS_LENGTH];
        let mut result = 0;

        for c in s.bytes() {
            let count = &mut counts[usize::from(c - b'A')];

            if *count {
                result += 2;
            }

            *count = !*count;
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
