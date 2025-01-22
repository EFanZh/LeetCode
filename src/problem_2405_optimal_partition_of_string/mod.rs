pub mod bit_manipulation;

pub trait Solution {
    fn partition_string(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abacaba", 4), ("ssssss", 6)];

        for (s, expected) in test_cases {
            assert_eq!(S::partition_string(s.to_string()), expected);
        }
    }
}
