pub mod dynamic_programming;

pub trait Solution {
    fn last_stone_weight_ii(stones: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 7, 4, 1, 8, 1] as &[_], 1), (&[31, 26, 33, 21, 40], 5)];

        for (stones, expected) in test_cases {
            assert_eq!(S::last_stone_weight_ii(stones.to_vec()), expected);
        }
    }
}
