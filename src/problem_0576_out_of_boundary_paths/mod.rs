pub mod dynamic_programming;

pub trait Solution {
    fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 2, 2, 0, 0), 6),
            ((1, 3, 3, 0, 1), 12),
            ((3, 2, 5, 1, 1), 109),
            ((8, 50, 23, 5, 26), 914_783_380),
        ];

        for ((m, n, max_move, start_row, start_column), expected) in test_cases {
            assert_eq!(S::find_paths(m, n, max_move, start_row, start_column), expected);
        }
    }
}
