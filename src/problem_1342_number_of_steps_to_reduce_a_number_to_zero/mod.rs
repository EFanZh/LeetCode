pub mod brute_force;
pub mod brute_force_2;

pub trait Solution {
    fn number_of_steps(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [(14, 6), (8, 4), (123, 12)];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::number_of_steps(num), expected);
        }
    }
}
