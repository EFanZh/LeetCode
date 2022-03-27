pub mod iterative;

pub trait Solution {
    fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, &[[1, 2]] as &[_]), 2),
            ((3, &[[1, 3], [2, 3]]), 3),
            ((3, &[[1, 3], [2, 3], [3, 1]]), -1),
            ((3, &[[1, 2], [2, 3]]), -1),
        ];

        for ((n, trust), expected) in test_cases {
            assert_eq!(
                S::find_judge(n, trust.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
