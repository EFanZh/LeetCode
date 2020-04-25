pub struct Solution {}

impl Solution {
    fn generate_parenthesis_unused(n: i32, unmatched: usize, buffer: &mut Vec<u8>, result: &mut Vec<String>) {
        buffer.push(b'(');
        Self::generate_parenthesis_unmatched(n - 1, unmatched + 1, buffer, result);

        if unmatched != 0 {
            *buffer.last_mut().unwrap() = b')';
            Self::generate_parenthesis_unused(n, unmatched - 1, buffer, result);
        }

        buffer.pop();
    }

    fn generate_parenthesis_unmatched(n: i32, unmatched: usize, buffer: &mut Vec<u8>, result: &mut Vec<String>) {
        if n == 0 {
            buffer.resize(buffer.len() + unmatched, b')');
            result.push(String::from_utf8(buffer.clone()).unwrap());
            buffer.truncate(buffer.len() - unmatched);
        } else {
            buffer.push(b'(');
            Self::generate_parenthesis_unmatched(n - 1, unmatched + 1, buffer, result);

            *buffer.last_mut().unwrap() = b')';
            Self::generate_parenthesis_unused(n, unmatched - 1, buffer, result);

            buffer.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        if n == 0 {
            result.push(String::new());
        } else {
            let mut buffer = vec![b'('];

            Self::generate_parenthesis_unmatched(n - 1, 1, &mut buffer, &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::generate_parenthesis(n)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
