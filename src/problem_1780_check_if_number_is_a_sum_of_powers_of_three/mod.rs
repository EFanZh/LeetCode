pub mod ternary_number;

pub trait Solution {
    fn check_powers_of_three(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(12, true), (91, true), (21, false)];

        for (n, expected) in test_cases {
            assert_eq!(S::check_powers_of_three(n), expected);
        }
    }
}
