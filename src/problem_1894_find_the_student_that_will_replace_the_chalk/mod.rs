pub mod iterative;

pub trait Solution {
    fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 1, 5] as &[_], 22), 0), ((&[3, 4, 1, 2], 25), 1)];

        for ((chalk, k), expected) in test_cases {
            assert_eq!(S::chalk_replacer(chalk.to_vec(), k), expected);
        }
    }
}
