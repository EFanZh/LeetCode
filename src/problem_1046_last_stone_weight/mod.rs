pub mod brute_force_with_binary_heap;

pub trait Solution {
    fn last_stone_weight(stones: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 7, 4, 1, 8, 1] as &[_], 1), (&[1], 1), (&[2, 2], 0)];

        for (stones, expected) in test_cases {
            assert_eq!(S::last_stone_weight(stones.to_vec()), expected);
        }
    }
}
