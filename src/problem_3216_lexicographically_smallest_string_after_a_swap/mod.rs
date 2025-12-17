pub mod greedy;

pub trait Solution {
    fn get_smallest_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("45320", "43520"), ("001", "001")];

        for (s, expected) in test_cases {
            assert_eq!(S::get_smallest_string(s.to_string()), expected);
        }
    }
}
