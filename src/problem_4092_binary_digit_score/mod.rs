pub mod brute_force;

pub trait Solution {
    fn calculate_score(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((14, 6), 6), ((5, 15), 2), ((10, 12), 4)];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::calculate_score(a, b), expected);
        }
    }
}
