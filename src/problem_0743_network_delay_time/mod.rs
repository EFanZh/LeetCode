pub mod dijkstra;
pub mod dijkstra_2;

pub trait Solution {
    fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 1, 1], [2, 3, 1], [3, 4, 1]] as &[_], 4, 2), 2),
            ((&[[1, 2, 1]], 2, 1), 1),
            ((&[[1, 2, 1]], 2, 2), -1),
            ((&[[1, 2, 1], [2, 3, 2], [1, 3, 4]], 3, 1), 3),
            ((&[[1, 2, 1], [2, 3, 7], [1, 3, 4], [2, 1, 2]], 3, 2), 6),
            (
                (
                    &[
                        [3, 5, 78],
                        [2, 1, 1],
                        [1, 3, 0],
                        [4, 3, 59],
                        [5, 3, 85],
                        [5, 2, 22],
                        [2, 4, 23],
                        [1, 4, 43],
                        [4, 5, 75],
                        [5, 1, 15],
                        [1, 5, 91],
                        [4, 1, 16],
                        [3, 2, 98],
                        [3, 4, 22],
                        [5, 4, 31],
                        [1, 2, 0],
                        [2, 5, 4],
                        [4, 2, 51],
                        [3, 1, 36],
                        [2, 3, 59],
                    ],
                    5,
                    5,
                ),
                31,
            ),
            (
                (
                    &[
                        [1, 5, 66],
                        [3, 5, 55],
                        [4, 3, 29],
                        [1, 2, 9],
                        [3, 4, 10],
                        [3, 1, 3],
                        [2, 3, 78],
                        [1, 4, 98],
                        [4, 5, 21],
                        [5, 2, 19],
                        [5, 1, 76],
                        [4, 1, 65],
                        [3, 2, 27],
                        [5, 3, 23],
                        [5, 4, 12],
                        [2, 1, 36],
                        [4, 2, 75],
                        [2, 4, 11],
                        [1, 3, 30],
                        [2, 5, 8],
                    ],
                    5,
                    1,
                ),
                30,
            ),
        ];

        for ((times, n, k), expected) in test_cases {
            assert_eq!(
                S::network_delay_time(times.iter().map(Vec::from).collect(), n, k),
                expected,
            );
        }
    }
}
