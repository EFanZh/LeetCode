pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut s = s.as_bytes();
        let mut result = 0;
        let mut stack = Vec::new();

        loop {
            if s.first().copied() == Some(b'(') {
                stack.push(result);

                s = &s[1..];
                result = 0;
            } else if let Some(top) = stack.pop() {
                s = &s[1..];
                result = top + if result == 0 { 1 } else { result * 2 };
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn score_of_parentheses(s: String) -> i32 {
        Self::score_of_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
