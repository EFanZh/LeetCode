pub mod dynamic_programming;

pub trait Solution {
    fn min_side_jumps(obstacles: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 2, 3, 0] as &[_], 2),
            (&[0, 1, 1, 3, 3, 0], 0),
            (&[0, 2, 1, 0, 3, 0], 2),
        ];

        for (obstacles, expected) in test_cases {
            assert_eq!(S::min_side_jumps(obstacles.to_vec()), expected);
        }
    }
}
