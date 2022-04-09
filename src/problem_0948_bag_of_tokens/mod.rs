pub mod greedy;

pub trait Solution {
    fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[100] as &[_], 50), 0),
            ((&[100, 200], 150), 1),
            ((&[100, 200, 300, 400], 200), 2),
            ((&[81, 91, 31], 73), 1),
        ];

        for ((tokens, power), expected) in test_cases {
            assert_eq!(S::bag_of_tokens_score(tokens.to_vec(), power), expected);
        }
    }
}
