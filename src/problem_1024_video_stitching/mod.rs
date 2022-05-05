pub mod iterative;

pub trait Solution {
    fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]] as &[_], 10), 3),
            ((&[[0, 1], [1, 2]], 5), -1),
            (
                (
                    &[
                        [0, 1],
                        [6, 8],
                        [0, 2],
                        [5, 6],
                        [0, 4],
                        [0, 3],
                        [6, 7],
                        [1, 3],
                        [4, 7],
                        [1, 4],
                        [2, 5],
                        [2, 6],
                        [3, 4],
                        [4, 5],
                        [5, 7],
                        [6, 9],
                    ],
                    9,
                ),
                3,
            ),
            ((&[[0, 2], [4, 8]], 5), -1),
        ];

        for ((clips, time), expected) in test_cases {
            assert_eq!(
                S::video_stitching(clips.iter().copied().map(Vec::from).collect(), time),
                expected
            );
        }
    }
}
