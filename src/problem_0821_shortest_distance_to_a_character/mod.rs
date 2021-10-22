pub mod iterative;

pub trait Solution {
    fn shortest_to_char(s: String, c: char) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("loveleetcode", 'e'), &[3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0] as &[_]),
            (("aaab", 'b'), &[3, 2, 1, 0]),
        ];

        for ((s, c), expected) in test_cases {
            assert_eq!(S::shortest_to_char(s.to_string(), c), expected);
        }
    }
}
