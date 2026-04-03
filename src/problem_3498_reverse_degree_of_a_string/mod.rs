pub mod iterative;

pub trait Solution {
    fn reverse_degree(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abc", 148), ("zaza", 160)];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_degree(s.to_string()), expected);
        }
    }
}
