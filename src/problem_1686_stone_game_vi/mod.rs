pub mod greedy;

pub trait Solution {
    fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3] as &[_], &[2, 1] as &[_]), 1),
            ((&[1, 2], &[3, 1]), 0),
            ((&[2, 4, 3], &[1, 6, 7]), -1),
        ];

        for ((alice_values, bob_values), expected) in test_cases {
            assert_eq!(S::stone_game_vi(alice_values.to_vec(), bob_values.to_vec()), expected);
        }
    }
}
