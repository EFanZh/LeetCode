pub mod bfs;

pub trait Solution {
    fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 4, 3, 1, 3, 2] as &[_], &[3, 2, 1, 3, 2, 4], &[2, 3, 3, 2, 3, 1]] as &[&[_]],
                4,
            ),
            (
                &[
                    &[12, 13, 1, 12],
                    &[13, 4, 13, 12],
                    &[13, 8, 10, 12],
                    &[12, 13, 12, 12],
                    &[13, 13, 13, 13],
                ],
                14,
            ),
            (
                &[
                    &[78, 16, 94, 36],
                    &[87, 93, 50, 22],
                    &[63, 28, 91, 60],
                    &[64, 27, 41, 27],
                    &[73, 37, 12, 69],
                    &[68, 30, 83, 31],
                    &[63, 24, 68, 36],
                ],
                44,
            ),
            (&[], 0),
        ];

        for (height_map, expected) in test_cases {
            assert_eq!(
                S::trap_rain_water(height_map.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
