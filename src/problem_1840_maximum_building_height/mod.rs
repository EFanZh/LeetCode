pub mod iterative;

pub trait Solution {
    fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[[2, 1], [4, 1]] as &[_]), 2),
            ((6, &[]), 5),
            ((10, &[[5, 3], [2, 5], [7, 4], [10, 3]]), 5),
        ];

        for ((n, restrictions), expected) in test_cases {
            assert_eq!(
                S::max_building(n, restrictions.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
