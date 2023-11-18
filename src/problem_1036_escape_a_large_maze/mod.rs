pub mod bfs;

pub trait Solution {
    fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 1], [1, 0]] as &[_], [0, 0], [0, 2]), false),
            ((&[], [0, 0], [999_999, 999_999]), true),
            ((&[[10, 9], [9, 10], [10, 11], [11, 10]], [0, 0], [10, 10]), false),
            ((&[[0, 3], [1, 0], [1, 1], [1, 2], [1, 3]], [0, 0], [0, 2]), true),
            (
                (
                    &[
                        [691_938, 300_406],
                        [710_196, 624_190],
                        [858_790, 609_485],
                        [268_029, 225_806],
                        [200_010, 188_664],
                        [132_599, 612_099],
                        [329_444, 633_495],
                        [196_657, 757_958],
                        [628_509, 883_388],
                    ],
                    [655_988, 180_910],
                    [267_728, 840_949],
                ),
                true,
            ),
        ];

        for ((blocked, source, target), expected) in test_cases {
            assert_eq!(
                S::is_escape_possible(
                    blocked.iter().map(Vec::from).collect(),
                    source.to_vec(),
                    target.to_vec()
                ),
                expected
            );
        }
    }
}
