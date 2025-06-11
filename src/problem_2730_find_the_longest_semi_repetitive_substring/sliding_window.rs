pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut start = 0;
        let mut repetition_count = 0;
        let mut prev = 0;

        for &c in s {
            repetition_count += u32::from(c == prev);

            if repetition_count > 1 {
                start += 1;
                repetition_count -= u32::from(s[start] == s[start - 1]);
            }

            prev = c;
        }

        (s.len() - start) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_semi_repetitive_substring(s: String) -> i32 {
        Self::longest_semi_repetitive_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
