pub mod stack;

pub trait Solution {
    fn min_remove_to_make_valid(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [("lee(t(c)o)de)", 12), ("a)b(c)d", 6), ("))((", 0)];

        for (s, expected) in test_cases {
            let result = S::min_remove_to_make_valid(s.to_string());

            assert_eq!(result.len(), expected);

            assert!(test_utilities::is_full_stack_operations(result.bytes().filter_map(
                |c| match c {
                    b'(' => Some(false),
                    b')' => Some(true),
                    _ => None,
                },
            )));

            assert!(test_utilities::is_subsequence(result.bytes(), s.bytes()));
        }
    }
}
