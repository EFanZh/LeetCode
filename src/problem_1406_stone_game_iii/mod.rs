pub mod dynamic_programming;

pub trait Solution {
    fn stone_game_iii(stone_value: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 7] as &[_], "Bob"),
            (&[1, 2, 3, -9], "Alice"),
            (&[1, 2, 3, 6], "Tie"),
            (&[-1, -2, -3], "Tie"),
        ];

        for (stone_value, expected) in test_cases {
            assert_eq!(S::stone_game_iii(stone_value.to_vec()), expected);
        }
    }
}
