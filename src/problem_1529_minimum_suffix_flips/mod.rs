pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn min_flips(target: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("10111", 3), ("101", 3), ("00000", 0)];

        for (target, expected) in test_cases {
            assert_eq!(S::min_flips(target.to_string()), expected);
        }
    }
}
