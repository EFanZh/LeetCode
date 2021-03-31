pub mod hash_set;

pub trait Solution {
    fn distribute_candies(candy_type: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 2, 2, 3, 3] as &[_], 3), (&[1, 1, 2, 3], 2), (&[6, 6, 6, 6], 1)];

        for (candy_type, expected) in test_cases.iter().copied() {
            assert_eq!(S::distribute_candies(candy_type.to_vec()), expected);
        }
    }
}
