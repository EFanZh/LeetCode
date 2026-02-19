pub mod hash_map;
pub mod hybrid_map;

pub trait Solution {
    fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcd", "cdab", 2), true),
            (("aabbcc", "bbaacc", 3), true),
            (("aabbcc", "bbaacc", 2), false),
        ];

        for ((s, t, k), expected) in test_cases {
            assert_eq!(S::is_possible_to_rearrange(s.to_string(), t.to_string(), k), expected);
        }
    }
}
