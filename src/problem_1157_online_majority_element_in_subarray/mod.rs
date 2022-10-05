pub mod segment_tree_and_binary_search;

pub trait MajorityChecker {
    fn new(arr: Vec<i32>) -> Self;
    fn query(&self, left: i32, right: i32, threshold: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::MajorityChecker;

    pub fn run<M: MajorityChecker>() {
        let test_cases = [
            (
                &[1, 1, 2, 2, 1, 1] as &[_],
                &[((0, 5, 4), 1), ((0, 3, 3), -1), ((2, 3, 2), 2)] as &[_],
            ),
            (
                &[2, 2, 1, 2, 1, 2, 2, 1, 1, 2],
                &[
                    ((2, 5, 4), -1),
                    ((0, 5, 6), -1),
                    ((0, 1, 2), 2),
                    ((2, 3, 2), -1),
                    ((6, 6, 1), 2),
                    ((0, 3, 3), 2),
                    ((4, 9, 6), -1),
                    ((4, 8, 4), -1),
                    ((5, 9, 5), -1),
                    ((0, 1, 2), 2),
                ],
            ),
        ];

        for (arr, queries) in test_cases {
            let majority_checker = M::new(arr.to_vec());

            for &((left, right, threshold), expected) in queries {
                assert_eq!(majority_checker.query(left, right, threshold), expected);
            }
        }
    }
}
