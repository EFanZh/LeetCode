pub mod iterative;

pub trait Solution {
    fn count_digits(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(7, 1), (121, 2), (1248, 4)];

        for (num, expected) in test_cases {
            assert_eq!(S::count_digits(num), expected);
        }
    }
}
