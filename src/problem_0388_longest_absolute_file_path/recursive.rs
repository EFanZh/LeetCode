pub struct Solution;

impl Solution {
    fn helper(input: &mut &[u8], depth: usize, parent_length: usize, result: &mut usize) {
        while input.get(..depth).map_or(false, |s| s.iter().all(|&c| c == b'\t')) {
            if let Some(name_length) = input[depth..].iter().position(|&c| c == b'\n') {
                if input[depth..depth + name_length].contains(&b'.') {
                    *input = &input[depth + name_length + 1..];
                    *result = (*result).max(parent_length + name_length);
                } else {
                    *input = &input[depth + name_length + 1..];

                    Self::helper(input, depth + 1, parent_length + name_length + 1, result);
                }
            } else {
                if input[depth..].contains(&b'.') {
                    *result = (*result).max(parent_length + input.len() - depth);
                }

                break;
            }
        }
    }

    pub fn length_longest_path(input: String) -> i32 {
        let mut result = 0;

        Self::helper(&mut input.as_bytes(), 0, 0, &mut result);

        result as _
    }
}

impl super::Solution for Solution {
    fn length_longest_path(input: String) -> i32 {
        Self::length_longest_path(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
