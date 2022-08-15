pub mod iterative;

pub trait Solution {
    fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[7, 4, 9] as &[_], [1, 2]), (&[6, 5, 4, 3, 10], [2, 3])];

        for (stones, expected) in test_cases {
            assert_eq!(S::num_moves_stones_ii(stones.to_vec()), expected);
        }
    }
}
