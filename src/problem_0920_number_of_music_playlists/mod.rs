pub mod dynamic_programming;

pub trait Solution {
    fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 3, 1), 6), ((2, 3, 0), 6), ((2, 3, 1), 2)];

        for ((n, goal, k), expected) in test_cases {
            assert_eq!(S::num_music_playlists(n, goal, k), expected);
        }
    }
}
