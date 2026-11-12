pub mod sliding_window;

pub trait Solution {
    fn shortest_beautiful_substring(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("100011001", 3), "11001"), (("1011", 2), "11"), (("000", 1), "")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::shortest_beautiful_substring(s.to_string(), k), expected);
        }
    }
}
