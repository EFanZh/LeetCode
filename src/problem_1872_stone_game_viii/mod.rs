pub mod dynamic_programming;

pub trait Solution {
    fn stone_game_viii(stones: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[-1, 2, -3, 4, -5] as &[_], 5),
            (&[7, -6, 5, 10, 5, -2, -6], 13),
            (&[-10, -12], -22),
        ];

        for (stones, expected) in test_cases {
            assert_eq!(S::stone_game_viii(stones.to_vec()), expected);
        }
    }
}
