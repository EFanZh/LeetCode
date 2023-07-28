pub mod greedy;

pub trait Solution {
    fn count_homogenous(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abbcccaa", 13), ("xy", 2), ("zzzzz", 15)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_homogenous(s.to_string()), expected);
        }
    }
}
