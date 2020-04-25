pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_length = 0;
        let mut stack = Vec::new();
        let mut cache = vec![0; s.len() + 1];

        for (i, c) in s.into_bytes().into_iter().enumerate() {
            if c == b'(' {
                stack.push(i);
            } else if let Some(j) = stack.pop() {
                let length = cache[j] + (i - j + 1);

                cache[i + 1] = length;
                max_length = max_length.max(length);
            }
        }

        max_length as _
    }
}

impl super::Solution for Solution {
    fn longest_valid_parentheses(s: String) -> i32 {
        Self::longest_valid_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
