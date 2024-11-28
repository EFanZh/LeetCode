pub mod recursive;

pub trait Solution {
    fn longest_path(parent: Vec<i32>, s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-1, 0, 0, 1, 1, 2] as &[_], "abacbe"), 3),
            ((&[-1, 0, 0, 0], "aabc"), 3),
        ];

        for ((parent, s), expected) in test_cases {
            assert_eq!(S::longest_path(parent.to_vec(), s.to_string()), expected);
        }
    }
}
