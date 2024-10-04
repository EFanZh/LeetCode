pub mod quick_select_three_way;

pub trait Solution {
    fn max_run_time(n: i32, batteries: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, &[3, 3, 3] as &[_]), 4),
            ((2, &[1, 1, 1, 1]), 2),
            ((3, &[10, 10, 6, 9, 3]), 12),
            (
                (12, &[11, 89, 16, 32, 70, 67, 35, 35, 31, 24, 41, 29, 6, 53, 78, 83]),
                43,
            ),
        ];

        for ((n, batteries), expected) in test_cases {
            assert_eq!(S::max_run_time(n, batteries.to_vec()), expected);
        }
    }
}
