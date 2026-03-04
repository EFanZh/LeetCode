pub mod binary_search;

pub trait Solution {
    fn min_length(s: String, num_ops: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("000001", 1), 2), (("0000", 2), 1), (("0101", 0), 1), (("1001", 1), 2)];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::min_length(s.to_string(), t), expected);
        }
    }
}
