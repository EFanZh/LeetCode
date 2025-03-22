pub mod greedy;

pub trait Solution {
    fn max_jump(stones: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 2, 5, 6, 7] as &[_], 5), (&[0, 3, 9], 9), (&[0, 3], 3)];

        for (stones, expected) in test_cases {
            assert_eq!(S::max_jump(stones.to_vec()), expected);
        }
    }
}
