pub mod division;

pub trait Solution {
    fn is_power_of_three(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(27, true), (0, false), (9, true), (45, false)];

        for (n, expected) in test_cases {
            assert_eq!(S::is_power_of_three(n), expected);
        }
    }
}
