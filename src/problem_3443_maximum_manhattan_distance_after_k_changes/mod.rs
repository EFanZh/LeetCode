pub mod greedy;

pub trait Solution {
    fn max_distance(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("NWSE", 1), 3), (("NSWWEW", 3), 6)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::max_distance(s.to_string(), k), expected);
        }
    }
}
