pub mod dynamic_programming;

pub trait Solution {
    fn max_profit(k: i32, prices: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, &[2, 4, 1] as &[_]), 2),
            ((2, &[3, 2, 6, 5, 0, 3]), 7),
            ((0, &[1, 3]), 0),
            ((2, &[]), 0),
            ((1, &[1]), 0),
        ];

        for ((k, prices), expected) in test_cases.iter().copied() {
            assert_eq!(S::max_profit(k, prices.to_vec()), expected);
        }
    }
}
