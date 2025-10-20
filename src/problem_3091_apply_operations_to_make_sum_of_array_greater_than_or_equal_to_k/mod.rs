pub mod mathematical;

pub trait Solution {
    fn min_operations(k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(11, 5), (1, 0)];

        for (k, expected) in test_cases {
            assert_eq!(S::min_operations(k), expected);
        }
    }
}
