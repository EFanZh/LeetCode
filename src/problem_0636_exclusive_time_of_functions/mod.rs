pub mod stack;

pub trait Solution {
    fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    1,
                    &["0:start:0", "0:start:2", "0:end:5", "0:start:6", "0:end:6", "0:end:7"] as &[_],
                ),
                &[8] as &[_],
            ),
            (
                (
                    2,
                    &["0:start:0", "0:start:2", "0:end:5", "1:start:6", "1:end:6", "0:end:7"],
                ),
                &[7, 1],
            ),
            (
                (
                    2,
                    &["0:start:0", "0:start:2", "0:end:5", "1:start:7", "1:end:7", "0:end:8"],
                ),
                &[8, 1],
            ),
            ((1, &["0:start:0", "0:end:0"]), &[1]),
        ];

        for ((n, logs), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::exclusive_time(n, logs.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
