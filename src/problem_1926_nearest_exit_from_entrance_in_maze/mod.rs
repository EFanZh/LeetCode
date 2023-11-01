pub mod bfs;

pub trait Solution {
    fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["++.+", "...+", "+++."] as &[_], [1, 2]), 1),
            ((&["+++", "...", "+++"], [1, 0]), 2),
            ((&[".+"], [0, 0]), -1),
        ];

        for ((maze, entrance), expected) in test_cases {
            assert_eq!(
                S::nearest_exit(maze.iter().map(|row| row.chars().collect()).collect(), entrance.into()),
                expected,
            );
        }
    }
}
