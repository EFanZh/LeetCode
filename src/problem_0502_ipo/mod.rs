pub mod greedy;

pub trait Solution {
    fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 0, &[1, 2, 3] as &[_], &[0, 1, 1] as &[_]), 4),
            ((3, 0, &[1, 2, 3], &[0, 1, 2]), 6),
            ((1, 0, &[1, 2, 3], &[1, 1, 2]), 0),
        ];

        for ((k, w, profits, capital), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_maximized_capital(k, w, profits.to_vec(), capital.to_vec()),
                expected
            );
        }
    }
}
