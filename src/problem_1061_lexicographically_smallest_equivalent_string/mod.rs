pub mod dfs;
pub mod union_find;

pub trait Solution {
    fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("parker", "morris", "parser"), "makkek"),
            (("hello", "world", "hold"), "hdld"),
            (("leetcode", "programs", "sourcecode"), "aauaaaaada"),
        ];

        for ((s1, s2, base_str), expected) in test_cases {
            assert_eq!(
                S::smallest_equivalent_string(s1.to_string(), s2.to_string(), base_str.to_string()),
                expected,
            );
        }
    }
}
