pub mod dynamic_programming;

pub trait Solution {
    fn stone_game_v(stone_value: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[6, 2, 3, 4, 5, 5] as &[_], 18),
            (&[7, 7, 7, 7, 7, 7, 7], 28),
            (&[4], 0),
            (&[9, 8, 2, 4, 6, 3, 5, 1, 7], 34),
            (&[98, 77, 24, 49, 6, 12, 2, 44, 51, 96], 330),
            (&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 37),
        ];

        for (stone_value, expected) in test_cases {
            assert_eq!(S::stone_game_v(stone_value.to_vec()), expected);
        }
    }
}
