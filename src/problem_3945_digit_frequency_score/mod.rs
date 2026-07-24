pub mod iterative;

pub trait Solution {
    fn digit_frequency_score(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(122, 5), (101, 2)];

        for (n, expected) in test_cases {
            assert_eq!(S::digit_frequency_score(n), expected);
        }
    }
}
