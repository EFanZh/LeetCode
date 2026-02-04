pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let result_length = target.bytes().map(usize::from).sum::<usize>() - usize::from(b'a' - 1) * target.len();
        let mut result = Vec::with_capacity(result_length);
        let mut s = String::with_capacity(target.len());

        for c in target.into_bytes() {
            s.push('a');
            result.push(s.clone());

            for c in b'b'..=c {
                s.pop();
                s.push(char::from(c));
                result.push(s.clone());
            }
        }

        assert!(result.len() == result_length);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn string_sequence(target: String) -> Vec<String> {
        Self::string_sequence(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
