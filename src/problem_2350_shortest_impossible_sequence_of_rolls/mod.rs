pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 2, 1, 2, 3, 3, 2, 4, 1] as &[_], 4), 3),
            ((&[1, 1, 2, 2], 2), 2),
            ((&[1, 1, 3, 2, 2, 2, 3, 3], 4), 1),
            (
                (
                    &[5, 6, 6, 2, 3, 6, 4, 6, 1, 5, 4, 5, 3, 3, 5, 3, 4, 3, 1, 3, 5, 4, 2],
                    6,
                ),
                2,
            ),
        ];

        for ((rolls, k), expected) in test_cases {
            assert_eq!(S::shortest_sequence(rolls.to_vec(), k), expected);
        }
    }
}
