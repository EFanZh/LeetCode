pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack = 0;

        for log in logs {
            match log.as_str() {
                "./" => {}
                "../" => stack -= i32::from(stack != 0),
                _ => stack += 1,
            }
        }

        stack
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(logs: Vec<String>) -> i32 {
        Self::min_operations(logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
