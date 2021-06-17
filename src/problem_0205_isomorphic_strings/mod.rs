pub mod direct_address_table;
pub mod hash_map;

pub trait Solution {
    fn is_isomorphic(s: String, t: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("egg", "add"), true),
            (("foo", "bar"), false),
            (("paper", "title"), true),
            (("ab", "aa"), false),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::is_isomorphic(s.to_string(), t.to_string()), expected);
        }
    }
}
