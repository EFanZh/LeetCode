pub mod dynamic_programming;

pub trait Solution {
    fn max_height(cuboids: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[50, 45, 20], [95, 37, 53], [45, 23, 12]] as &[_], 190),
            (&[[38, 25, 45], [76, 35, 3]], 76),
            (
                &[
                    [7, 11, 17],
                    [7, 17, 11],
                    [11, 7, 17],
                    [11, 17, 7],
                    [17, 7, 11],
                    [17, 11, 7],
                ],
                102,
            ),
            (
                &[
                    [92, 47, 83],
                    [75, 20, 87],
                    [68, 12, 83],
                    [12, 85, 15],
                    [16, 24, 47],
                    [69, 65, 35],
                    [96, 56, 93],
                    [89, 93, 11],
                    [86, 20, 41],
                    [69, 77, 12],
                    [83, 80, 97],
                    [90, 22, 36],
                ],
                447,
            ),
            (
                &[
                    [74, 66, 55],
                    [97, 97, 38],
                    [21, 69, 43],
                    [88, 83, 87],
                    [11, 4, 96],
                    [17, 98, 21],
                    [45, 43, 12],
                    [19, 67, 24],
                    [72, 5, 81],
                    [30, 53, 100],
                    [38, 30, 29],
                    [81, 53, 42],
                    [78, 80, 9],
                    [3, 80, 66],
                    [74, 16, 92],
                    [18, 17, 70],
                    [66, 88, 56],
                    [7, 51, 49],
                    [9, 59, 13],
                    [44, 67, 21],
                    [9, 95, 14],
                    [88, 100, 37],
                    [23, 76, 24],
                    [15, 38, 41],
                    [47, 98, 66],
                    [25, 39, 23],
                    [74, 49, 28],
                    [100, 82, 62],
                    [96, 73, 52],
                    [9, 22, 5],
                    [83, 99, 28],
                    [9, 35, 5],
                    [26, 53, 33],
                    [53, 98, 93],
                ],
                605,
            ),
        ];

        for (cuboids, expected) in test_cases {
            assert_eq!(
                S::max_height(cuboids.iter().copied().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
