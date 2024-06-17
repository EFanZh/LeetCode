pub mod iterative;

pub trait Solution {
    fn truncate_sentence(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("Hello how are you Contestant", 4), "Hello how are you"),
            (("What is the solution to this problem", 4), "What is the solution"),
            (("chopper is not a tanuki", 5), "chopper is not a tanuki"),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::truncate_sentence(s.to_string(), k), expected);
        }
    }
}
