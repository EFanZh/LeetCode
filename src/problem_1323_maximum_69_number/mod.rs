pub mod iterative;

pub trait Solution {
    fn maximum69_number(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(9669, 9969), (9996, 9999), (9999, 9999)];

        for (num, expected) in test_cases {
            assert_eq!(S::maximum69_number(num), expected);
        }
    }
}
