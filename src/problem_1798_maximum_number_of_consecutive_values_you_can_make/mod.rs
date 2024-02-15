pub mod greedy;

pub trait Solution {
    fn get_maximum_consecutive(coins: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3] as &[_], 2), (&[1, 1, 1, 4], 8), (&[1, 4, 10, 3, 1], 20)];

        for (coins, expected) in test_cases {
            assert_eq!(S::get_maximum_consecutive(coins.to_vec()), expected);
        }
    }
}
