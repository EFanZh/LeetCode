pub mod union_find;

pub trait Solution {
    fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[1, 2, 9], [2, 3, 6], [2, 4, 5], [1, 4, 7]] as &[_]), 5),
            ((4, &[[1, 2, 2], [1, 3, 4], [3, 4, 7]]), 2),
            (
                (
                    36,
                    &[
                        [7, 11, 418],
                        [13, 23, 287],
                        [16, 25, 7891],
                        [15, 7, 9695],
                        [4, 3, 9569],
                        [17, 7, 1809],
                        [14, 3, 4720],
                        [14, 4, 6118],
                        [9, 2, 4290],
                        [32, 17, 5645],
                        [14, 16, 426],
                        [36, 7, 6721],
                        [13, 30, 9444],
                        [3, 25, 4635],
                        [33, 5, 1669],
                        [22, 18, 8910],
                        [5, 28, 7865],
                        [13, 10, 9466],
                        [7, 9, 2457],
                        [11, 8, 4711],
                        [17, 11, 6308],
                        [7, 34, 3789],
                        [8, 33, 9659],
                        [16, 3, 4187],
                        [16, 20, 3595],
                        [23, 10, 6251],
                        [26, 22, 6180],
                        [4, 16, 5577],
                        [26, 7, 5398],
                        [6, 36, 8671],
                        [10, 19, 3028],
                        [23, 30, 1330],
                        [19, 13, 8315],
                        [25, 20, 4740],
                        [25, 4, 5818],
                        [30, 10, 8030],
                        [30, 19, 7527],
                        [28, 6, 6804],
                        [21, 27, 1746],
                        [18, 9, 5189],
                        [7, 27, 6560],
                        [20, 14, 2450],
                        [27, 32, 3951],
                        [2, 21, 3927],
                        [1, 15, 9283],
                        [3, 20, 5428],
                        [15, 26, 5871],
                        [19, 23, 4533],
                        [14, 25, 6992],
                        [4, 20, 5831],
                    ],
                ),
                418,
            ),
            (
                (
                    6,
                    &[
                        [4, 5, 7468],
                        [6, 2, 7173],
                        [6, 3, 8365],
                        [2, 3, 7674],
                        [5, 6, 7852],
                        [1, 2, 8547],
                        [2, 4, 1885],
                        [2, 5, 5192],
                        [1, 3, 4065],
                        [1, 4, 7357],
                    ],
                ),
                1885,
            ),
        ];

        for ((n, roads), expected) in test_cases {
            assert_eq!(S::min_score(n, roads.iter().map(Vec::from).collect()), expected);
        }
    }
}
