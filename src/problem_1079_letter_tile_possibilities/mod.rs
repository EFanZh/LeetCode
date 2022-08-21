pub mod brute_force;
pub mod mathematical;

pub trait Solution {
    fn num_tile_possibilities(tiles: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("AAB", 8), ("AAABBC", 188), ("V", 1), ("ABCDEFG", 13699)];

        for (tiles, expected) in test_cases {
            assert_eq!(S::num_tile_possibilities(tiles.to_string()), expected);
        }
    }
}
