pub mod dynamic_programming;

pub trait Solution {
    fn num_of_arrays(n: i32, m: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 3, 1), 6),
            ((5, 2, 3), 0),
            ((9, 1, 1), 1),
            ((5, 3, 2), 120),
            ((50, 100, 25), 34_549_172),
        ];

        for ((n, m, k), expected) in test_cases {
            assert_eq!(S::num_of_arrays(n, m, k), expected);
        }
    }
}
