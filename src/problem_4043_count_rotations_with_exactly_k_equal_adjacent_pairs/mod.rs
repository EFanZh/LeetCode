pub mod iterative;

pub trait Solution {
    fn count_rotations(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aab", 1), 2), (("abca", 0), 1)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::count_rotations(s.to_string(), k), expected);
        }
    }
}
