pub mod dynamic_programming;

pub trait Solution {
    fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 1, 2, 2, 3, 3, 5] as &[_], 2), 7), ((&[1, 2, 3, 4], 0), 2)];

        for ((prize_positions, k), expected) in test_cases {
            assert_eq!(S::maximize_win(prize_positions.to_vec(), k), expected);
        }
    }
}
