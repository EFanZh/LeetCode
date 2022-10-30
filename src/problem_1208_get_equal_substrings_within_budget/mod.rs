pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn equal_substring(s: String, t: String, max_cost: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcd", "bcdf", 3), 3),
            (("abcd", "cdef", 3), 1),
            (("abcd", "acde", 0), 1),
            (("krrgw", "zjxss", 19), 2),
        ];

        for ((s, t, max_cost), expected) in test_cases {
            assert_eq!(S::equal_substring(s.to_string(), t.to_string(), max_cost), expected);
        }
    }
}
