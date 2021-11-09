pub mod iterative;

pub trait Solution {
    fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 4, 4, 7] as &[_], &[1, 3, 4, 1, 3] as &[_]), 2),
            ((&[1], &[1]), 0),
        ];

        for ((fronts, backs), expected) in test_cases {
            assert_eq!(S::flipgame(fronts.to_vec(), backs.to_vec()), expected);
        }
    }
}
