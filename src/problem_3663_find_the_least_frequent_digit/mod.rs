pub mod mathematical;

pub trait Solution {
    fn get_least_frequent_digit(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1_553_322, 1), (723_344_511, 2)];

        for (n, expected) in test_cases {
            assert_eq!(S::get_least_frequent_digit(n), expected);
        }
    }
}
