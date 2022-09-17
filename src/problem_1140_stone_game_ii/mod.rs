pub mod dynamic_programming;

pub trait Solution {
    fn stone_game_ii(piles: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 7, 9, 4, 4] as &[_], 10), (&[1, 2, 3, 4, 5, 100], 104)];

        for (piles, expected) in test_cases {
            assert_eq!(S::stone_game_ii(piles.to_vec()), expected);
        }
    }
}
