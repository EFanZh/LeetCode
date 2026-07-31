pub mod iterative;

pub trait Solution {
    fn rearrange_string(s: String, x: char, y: char) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [("aabc", 'a', 'c'), ("dcab", 'd', 'b'), ("axe", 'o', 'x')];

        for (s, x, y) in test_cases {
            let result = S::rearrange_string(s.to_string(), x, y);

            assert_eq!(
                test_utilities::unstable_sorted(result.bytes()),
                test_utilities::unstable_sorted(s.bytes()),
            );

            let mut iter = result.chars();

            assert!(iter.all(|c| c != x) || iter.all(|c| c != y));
        }
    }
}
