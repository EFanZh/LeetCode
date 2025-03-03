pub mod greedy;

pub trait Solution {
    fn make_integer_beautiful(n: i64, target: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((16, 6), 4), ((467, 6), 33), ((1, 1), 0), ((94598, 6), 5402)];

        for ((n, target), expected) in test_cases {
            assert_eq!(S::make_integer_beautiful(n, target), expected);
        }
    }
}
