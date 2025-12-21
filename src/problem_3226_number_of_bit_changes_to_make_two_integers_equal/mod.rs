pub mod greedy;

pub trait Solution {
    fn min_changes(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((13, 4), 2), ((21, 21), 0), ((14, 13), -1)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::min_changes(n, k), expected);
        }
    }
}
