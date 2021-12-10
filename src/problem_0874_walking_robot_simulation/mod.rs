pub mod iterative;

pub trait Solution {
    fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, -1, 3] as &[_], &[] as &[[_; 2]]), 25),
            ((&[4, -1, 4, -2, 4], &[[2, 4]]), 65),
            ((&[6, -1, -1, 6], &[]), 36),
        ];

        for ((commands, obstacles), expected) in test_cases {
            assert_eq!(
                S::robot_sim(commands.to_vec(), obstacles.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
