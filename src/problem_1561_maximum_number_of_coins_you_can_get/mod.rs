pub mod greedy;

pub trait Solution {
    fn max_coins(piles: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 4, 1, 2, 7, 8] as &[_], 9),
            (&[2, 4, 5], 4),
            (&[9, 8, 7, 6, 5, 1, 2, 3, 4], 18),
        ];

        for (piles, expected) in test_cases {
            assert_eq!(S::max_coins(piles.to_vec()), expected);
        }
    }
}
