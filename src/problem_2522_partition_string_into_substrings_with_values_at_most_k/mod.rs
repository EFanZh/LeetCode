pub mod greedy;

pub trait Solution {
    fn minimum_partition(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("165462", 60), 4), (("238182", 5), -1), (("1", 1), 1)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::minimum_partition(s.to_string(), k), expected);
        }
    }
}
