pub mod bfs;
pub mod bidirectional_bfs;
pub mod dfs;
pub mod pseudo_dfs;
pub mod union_find;

pub trait Solution {
    fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[0, 1], [1, 2], [2, 0]] as &[_], 0, 2), true),
            ((6, &[[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]], 0, 5), false),
            ((1, &[], 0, 0), true),
            (
                (
                    10,
                    &[
                        [4, 3],
                        [1, 4],
                        [4, 8],
                        [1, 7],
                        [6, 4],
                        [4, 2],
                        [7, 4],
                        [4, 0],
                        [0, 9],
                        [5, 4],
                    ],
                    5,
                    9,
                ),
                true,
            ),
            (
                (
                    50,
                    &[
                        [31, 5],
                        [10, 46],
                        [19, 31],
                        [5, 1],
                        [31, 28],
                        [28, 29],
                        [8, 26],
                        [13, 23],
                        [16, 34],
                        [30, 1],
                        [16, 18],
                        [33, 46],
                        [27, 35],
                        [2, 25],
                        [49, 33],
                        [44, 19],
                        [22, 26],
                        [30, 13],
                        [27, 12],
                        [8, 16],
                        [42, 13],
                        [18, 3],
                        [21, 20],
                        [2, 17],
                        [5, 48],
                        [41, 37],
                        [39, 37],
                        [2, 11],
                        [20, 26],
                        [19, 43],
                        [45, 7],
                        [0, 21],
                        [44, 23],
                        [2, 39],
                        [27, 36],
                        [41, 48],
                        [17, 42],
                        [40, 32],
                        [2, 28],
                        [35, 38],
                        [3, 9],
                        [41, 30],
                        [5, 11],
                        [24, 22],
                        [39, 5],
                        [40, 31],
                        [18, 35],
                        [23, 39],
                        [20, 24],
                        [45, 12],
                    ],
                    29,
                    46,
                ),
                false,
            ),
            ((7, &[[0, 1], [1, 2], [1, 3], [4, 5], [5, 6]], 0, 4), false),
        ];

        for ((n, edges, source, destination), expected) in test_cases {
            assert_eq!(
                S::valid_path(n, edges.iter().map(Vec::from).collect(), source, destination),
                expected,
            );
        }
    }
}
