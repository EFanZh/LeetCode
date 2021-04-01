pub mod divide_and_conquer;
pub mod ordered_map;
pub mod priority_queue;

pub trait Solution {
    fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[2, 9, 10], [3, 7, 15], [5, 12, 12], [15, 20, 10], [19, 24, 8]] as &[_],
                &[[2, 10], [3, 15], [7, 12], [12, 0], [15, 10], [20, 8], [24, 0]] as &[_],
            ),
            (&[[1, 2, 1], [1, 2, 2], [1, 2, 3]], &[[1, 3], [2, 0]]),
            (&[[0, 2, 3], [2, 5, 3]], &[[0, 3], [5, 0]]),
            (
                &[
                    [0, 5, 7],
                    [5, 10, 7],
                    [5, 10, 12],
                    [10, 15, 7],
                    [15, 20, 7],
                    [15, 20, 12],
                    [20, 25, 7],
                ],
                &[[0, 7], [5, 12], [10, 7], [15, 12], [20, 7], [25, 0]],
            ),
        ];

        for (buildings, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::get_skyline(buildings.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
