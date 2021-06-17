pub mod recursive;

pub trait Solution {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (4, 2),
            &[&[1, 2] as &[_], &[1, 3], &[1, 4], &[2, 3], &[2, 4], &[3, 4]] as &[&[_]],
        )];

        for ((n, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::combine(n, k).into_iter().map(test_utilities::unstable_sorted)),
                expected
            );
        }
    }
}
