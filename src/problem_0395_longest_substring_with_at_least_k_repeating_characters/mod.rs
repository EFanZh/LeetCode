pub mod divide_and_conquer;
pub mod sliding_window;

pub trait Solution {
    fn longest_substring(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aaabb", 3), 3), (("ababbc", 2), 5), (("ababacb", 3), 0)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::longest_substring(s.to_string(), k), expected);
        }
    }
}
