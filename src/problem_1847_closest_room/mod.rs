pub mod monotonic_stack;

pub trait Solution {
    fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[2, 2], [1, 2], [3, 2]] as &[_], &[[3, 1], [3, 3], [5, 2]] as &[_]),
                &[3, -1, 3] as &[_],
            ),
            (
                (&[[1, 4], [2, 3], [3, 5], [4, 1], [5, 2]], &[[2, 3], [2, 4], [2, 5]]),
                &[2, 1, 3],
            ),
            (
                (
                    &[[10, 40], [20, 30], [30, 50], [40, 10], [50, 20]],
                    &[[2, 3], [2, 4], [2, 5]],
                ),
                &[10, 10, 10],
            ),
        ];

        for ((rooms, queries), expected) in test_cases {
            assert_eq!(
                S::closest_room(
                    rooms.iter().map(Vec::from).collect(),
                    queries.iter().map(Vec::from).collect()
                ),
                expected,
            );
        }
    }
}
