pub mod brute_force;

pub trait Solution {
    fn sum_of_number_and_reverse(num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(443, true), (63, false), (181, true)];
        for (num, expected) in test_cases {
            assert_eq!(S::sum_of_number_and_reverse(num), expected);
        }
    }
}
