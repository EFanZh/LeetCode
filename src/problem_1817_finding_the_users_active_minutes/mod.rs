pub mod hash_map_and_hash_set;

pub trait Solution {
    fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[0, 5], [1, 2], [0, 2], [0, 5], [1, 3]] as &[_], 5),
                &[0, 2, 0, 0, 0] as &[_],
            ),
            ((&[[1, 1], [2, 2], [2, 3]], 4), &[1, 1, 0, 0]),
        ];

        for ((logs, k), expected) in test_cases {
            assert_eq!(
                S::finding_users_active_minutes(logs.iter().copied().map(Vec::from).collect(), k),
                expected,
            );
        }
    }
}
