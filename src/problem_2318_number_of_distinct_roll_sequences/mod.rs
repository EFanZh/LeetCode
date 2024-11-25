pub mod dynamic_programming;

pub trait Solution {
    fn distinct_sequences(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(0, 1), (1, 6), (2, 22), (3, 66), (4, 184)];

        for (n, expected) in test_cases {
            assert_eq!(S::distinct_sequences(n), expected);
        }
    }
}
