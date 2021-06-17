pub mod solve_equations;

pub trait Solution {
    fn original_digits(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("owoztneoer", "012"), ("fviefuro", "45"), ("xsi", "6"), ("nnei", "9")];

        for (s, expected) in test_cases {
            assert_eq!(S::original_digits(s.to_string()), expected);
        }
    }
}
