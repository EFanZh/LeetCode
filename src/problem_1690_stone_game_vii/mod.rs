pub mod dynamic_programming;

pub trait Solution {
    fn stone_game_vii(stones: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 3, 1, 4, 2] as &[_], 6), (&[7, 90, 5, 1, 100, 10, 10, 2], 122)];

        for (stones, expected) in test_cases {
            assert_eq!(S::stone_game_vii(stones.to_vec()), expected);
        }
    }
}
