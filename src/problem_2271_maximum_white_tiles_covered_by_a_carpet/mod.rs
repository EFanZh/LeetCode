pub mod sliding_window;

pub trait Solution {
    fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 5], [10, 11], [12, 18], [20, 25], [30, 32]] as &[_], 10), 9),
            ((&[[10, 11], [1, 1]], 2), 2),
        ];

        for ((tiles, carpet_len), expected) in test_cases {
            assert_eq!(
                S::maximum_white_tiles(tiles.iter().map(Vec::from).collect(), carpet_len),
                expected,
            );
        }
    }
}
