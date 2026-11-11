pub mod iterative;

pub trait Solution {
    fn mirror_frequency(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("ab1z9", 3), ("4m7n", 2), ("byby", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::mirror_frequency(s.to_string()), expected);
        }
    }
}
