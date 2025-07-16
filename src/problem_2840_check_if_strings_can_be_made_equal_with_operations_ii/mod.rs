pub mod count_letters;

pub trait Solution {
    fn check_strings(s1: String, s2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abcdba", "cabdab"), true), (("abe", "bea"), false)];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::check_strings(s1.to_string(), s2.to_string()), expected);
        }
    }
}
