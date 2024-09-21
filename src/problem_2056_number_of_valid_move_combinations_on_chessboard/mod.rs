pub mod brute_force;

pub trait Solution {
    fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["rook"] as &[_], &[[1, 1]] as &[_]), 15),
            ((&["queen"], &[[1, 1]]), 22),
            ((&["bishop"], &[[4, 3]]), 12),
            ((&["queen", "bishop"], &[[5, 7], [3, 4]]), 281),
            ((&["queen", "rook"], &[[1, 2], [7, 2]]), 293),
        ];

        for ((pieces, positions), expected) in test_cases {
            assert_eq!(
                S::count_combinations(
                    pieces.iter().copied().map(str::to_string).collect(),
                    positions.iter().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}
