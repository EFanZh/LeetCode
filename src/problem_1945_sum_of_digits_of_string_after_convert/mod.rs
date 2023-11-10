pub mod iterative;

pub trait Solution {
    fn get_lucky(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("iiii", 1), 36), (("leetcode", 2), 6), (("zbax", 2), 8)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::get_lucky(s.to_string(), k), expected);
        }
    }
}
