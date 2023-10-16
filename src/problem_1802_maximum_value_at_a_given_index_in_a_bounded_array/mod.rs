pub mod mathematical;

pub trait Solution {
    fn max_value(n: i32, index: i32, max_sum: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((4, 2, 6), 2), ((6, 1, 10), 3), ((4, 0, 4), 1), ((5, 0, 28), 7)];

        for ((n, index, max_sum), expected) in test_cases {
            assert_eq!(S::max_value(n, index, max_sum), expected);
        }
    }
}
