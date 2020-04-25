pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec![String::new()];
        } else {
            let mut result = Vec::new();

            for i in (0..n).rev() {
                for left in Self::generate_parenthesis(i) {
                    for right in Self::generate_parenthesis(n - i - 1) {
                        result.push(format!("({}){}", left, right));
                    }
                }
            }

            result
        }
    }
}

impl super::Solution for Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::generate_parenthesis(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
