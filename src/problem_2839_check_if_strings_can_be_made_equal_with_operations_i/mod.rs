pub mod sorting;

pub trait Solution {
    fn can_be_equal(s1: String, s2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abcd", "cdab"), true), (("abcd", "dacb"), false)];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::can_be_equal(s1.to_string(), s2.to_string()), expected);
        }
    }
}
