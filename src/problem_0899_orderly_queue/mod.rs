pub mod duval;

pub trait Solution {
    fn orderly_queue(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("cba", 1), "acb"), (("baaca", 3), "aaabc")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::orderly_queue(s.to_string(), k), expected.to_string());
        }
    }
}
