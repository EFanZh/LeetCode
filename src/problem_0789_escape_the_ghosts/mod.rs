pub mod mathematical;

pub trait Solution {
    fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 0], [0, 3]] as &[_], [0, 1]), true),
            ((&[[1, 0]], [2, 0]), false),
            ((&[[2, 0]], [1, 0]), false),
            ((&[[5, 0], [-10, -2], [0, -5], [-2, -2], [-7, 1]], [7, 7]), false),
            ((&[[-1, 0], [0, 1], [-1, 0], [0, 1], [-1, 0]], [0, 0]), true),
        ];

        for ((ghosts, target), expected) in test_cases {
            assert_eq!(
                S::escape_ghosts(ghosts.iter().copied().map(Vec::from).collect(), target.to_vec()),
                expected
            );
        }
    }
}
