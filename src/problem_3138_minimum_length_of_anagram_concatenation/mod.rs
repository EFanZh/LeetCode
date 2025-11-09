pub mod brute_force;

pub trait Solution {
    fn min_anagram_length(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abba", 2), ("cdef", 4), ("abcbcacabbaccba", 3), ("aabb", 4)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_anagram_length(s.to_string()), expected);
        }
    }
}
