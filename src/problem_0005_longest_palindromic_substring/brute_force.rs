pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut max_left = 0;
        let mut max_length = 0;

        let mut update_result = |mut i: usize, mut j: usize| {
            while let (Some(lhs), Some(rhs)) = (bytes.get(i), bytes.get(j)) {
                if lhs == rhs {
                    i = i.wrapping_sub(1);
                    j += 1;
                } else {
                    break;
                }
            }

            let length = j.wrapping_sub(i) - 1;

            if length > max_length {
                max_left = i.wrapping_add(1);
                max_length = length;
            }
        };

        for i in 0..bytes.len() {
            update_result(i.wrapping_sub(1), i + 1);
            update_result(i, i + 1);
        }

        s[max_left..max_left + max_length].to_string()
    }
}

impl super::Solution for Solution {
    fn longest_palindrome(s: String) -> String {
        Self::longest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
