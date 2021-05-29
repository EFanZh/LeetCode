pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn generate_parenthesis_helper(n: i32, unmatched: usize, buffer: &mut Vec<u8>, result: &mut Vec<String>) {
        if n == 0 {
            buffer.resize(buffer.len() + unmatched, b')');
            result.push(String::from_utf8(buffer.clone()).unwrap());
            buffer.truncate(buffer.len() - unmatched);
        } else {
            buffer.push(b'(');

            Self::generate_parenthesis_helper(n - 1, unmatched + 1, buffer, result);

            if unmatched > 0 {
                *buffer.last_mut().unwrap() = b')';

                Self::generate_parenthesis_helper(n, unmatched - 1, buffer, result);
            }

            buffer.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut buffer = Vec::new();

        Self::generate_parenthesis_helper(n, 0, &mut buffer, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
