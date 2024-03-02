pub mod mathematical;

pub trait Solution {
    fn minimum_sum(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(2932, 52), (4009, 13)];

        for (num, expected) in test_cases {
            assert_eq!(S::minimum_sum(num), expected);
        }
    }
}
