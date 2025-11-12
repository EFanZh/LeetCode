pub mod greedy;

pub trait Solution {
    fn find_permutation_difference(s: String, t: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abc", "bac"), 2), (("abcde", "edbac"), 12)];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::find_permutation_difference(s.to_string(), t.to_string()), expected);
        }
    }
}
