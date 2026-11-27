pub mod greedy;

pub trait Solution {
    fn min_range(a: i32, b: i32, c: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((9, 2, 5), 5), ((5, 7, 6), 2), ((3, 3, 3), 0)];

        for ((a, b, c), expected) in test_cases {
            assert_eq!(S::min_range(a, b, c), expected);
        }
    }
}
