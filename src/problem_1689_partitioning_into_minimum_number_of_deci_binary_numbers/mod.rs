pub mod greedy;

pub trait Solution {
    fn min_partitions(n: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("32", 3), ("82734", 8), ("27346209830709182346", 9)];

        for (n, expected) in test_cases {
            assert_eq!(S::min_partitions(n.to_string()), expected);
        }
    }
}
