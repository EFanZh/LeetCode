pub mod iterative;

pub trait Solution {
    fn remove_stars(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leet**cod*e", "lecoe"), ("erase*****", ""), ("abcd", "abcd")];

        for (s, expected) in test_cases {
            assert_eq!(S::remove_stars(s.to_string()), expected);
        }
    }
}
