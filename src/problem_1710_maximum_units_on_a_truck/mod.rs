pub mod greedy_binary_heap;
pub mod greedy_quick_select;

pub trait Solution {
    fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 3], [2, 2], [3, 1]] as &[_], 4), 8),
            ((&[[5, 10], [2, 5], [4, 7], [3, 9]], 10), 91),
            (
                (
                    &[
                        [2, 1],
                        [4, 4],
                        [3, 1],
                        [4, 1],
                        [2, 4],
                        [3, 4],
                        [1, 3],
                        [4, 3],
                        [5, 3],
                        [5, 3],
                    ],
                    13,
                ),
                48,
            ),
        ];

        for ((box_types, truck_size), expected) in test_cases {
            assert_eq!(
                S::maximum_units(box_types.iter().map(Vec::from).collect(), truck_size),
                expected,
            );
        }
    }
}
