pub mod iterative;

pub trait Solution {
    fn check_distances(s: String, distance: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    "abaccb",
                    [
                        1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                true,
            ),
            (
                (
                    "aa",
                    [
                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                false,
            ),
        ];

        for ((s, distance), expected) in test_cases {
            assert_eq!(S::check_distances(s.to_string(), distance.to_vec()), expected);
        }
    }
}
