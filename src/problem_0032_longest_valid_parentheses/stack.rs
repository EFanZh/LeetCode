pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_length = 0;
        let mut stack_base = -1;
        let mut stack = Vec::new();

        for (i, c) in (0..).zip(s.into_bytes()) {
            if c == b'(' {
                stack.push(i);
            } else if stack.pop().is_none() {
                stack_base = i;
            } else {
                max_length = max_length.max(i - stack.last().copied().unwrap_or(stack_base));
            }
        }

        max_length
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
