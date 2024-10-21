pub mod dynamic_programming;

pub trait Solution {
    fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((9, [1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]), 47),
            ((3, [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]), 27),
        ];

        for ((num_arrows, alice_arrows), expected) in test_cases {
            let result = S::maximum_bob_points(num_arrows, alice_arrows.to_vec());
            let result: &[_; 12] = result.as_slice().try_into().unwrap();

            assert_eq!(result.iter().sum::<i32>(), num_arrows);

            assert_eq!(
                (0..)
                    .zip(result.iter().zip(&alice_arrows))
                    .fold(0, |score, (i, (bob_count, alice_count))| {
                        score + (if bob_count > alice_count { i } else { 0 })
                    }),
                expected,
            );
        }
    }
}
