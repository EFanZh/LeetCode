pub mod dynamic_programming;

pub trait Solution {
    fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[&[1, 100, 3] as &[_], &[7, 8, 9]] as &[&[_]], 2), 101),
            (
                (
                    &[&[100], &[100], &[100], &[100], &[100], &[100], &[1, 1, 1, 1, 1, 1, 700]],
                    7,
                ),
                706,
            ),
        ];

        for ((piles, k), expected) in test_cases {
            assert_eq!(
                S::max_value_of_coins(piles.iter().copied().map(Vec::from).collect(), k),
                expected,
            );
        }
    }
}
