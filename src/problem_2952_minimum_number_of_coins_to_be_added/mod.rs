pub mod greedy;

pub trait Solution {
    fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 4, 10] as &[_], 19), 2),
            ((&[1, 4, 10, 5, 7, 19], 19), 1),
            ((&[1, 1, 1], 20), 3),
        ];

        for ((coins, target), expected) in test_cases {
            assert_eq!(S::minimum_added_coins(coins.to_vec(), target), expected);
        }
    }
}
