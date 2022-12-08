pub mod greedy;

pub trait Solution {
    fn min_taps(n: i32, ranges: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[3, 4, 1, 1, 0, 0] as &[_]), 1),
            ((3, &[0, 0, 0, 0]), -1),
            ((7, &[1, 2, 1, 0, 2, 1, 0, 1]), 3),
        ];

        for ((n, ranges), expected) in test_cases {
            assert_eq!(S::min_taps(n, ranges.to_vec()), expected);
        }
    }
}
