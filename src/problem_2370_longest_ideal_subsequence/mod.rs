pub mod dynamic_programming;

pub trait Solution {
    fn longest_ideal_string(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("acfgbd", 2), 4), (("abcd", 3), 4)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::longest_ideal_string(s.to_string(), k), expected);
        }
    }
}
