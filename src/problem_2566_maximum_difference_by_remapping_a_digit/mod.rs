pub mod greedy;

pub trait Solution {
    fn min_max_difference(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(11891, 99009), (90, 99), (99999, 99999)];

        for (num, expected) in test_cases {
            assert_eq!(S::min_max_difference(num), expected);
        }
    }
}
