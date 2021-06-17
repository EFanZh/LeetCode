pub mod formula;

pub trait Solution {
    fn add_digits(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(38, 2)];

        for (num, expected) in test_cases {
            assert_eq!(S::add_digits(num), expected);
        }
    }
}
