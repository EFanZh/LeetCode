pub mod bfs;

pub trait Solution {
    fn min_push_box(grid: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["######", "#T####", "#..B.#", "#.##.#", "#...S#", "######"] as &[_], 3),
            (&["######", "#T####", "#..B.#", "####.#", "#...S#", "######"], -1),
            (&["######", "#T..##", "#.#B.#", "#....#", "#...S#", "######"], 5),
            (
                &["#..#####", "#..T#..#", "#...#B.#", "#......#", "#...#.S#", "#..#####"],
                7,
            ),
            (&["..#..", "####.", "#....", "#...#", "...B.", "TS..#"], -1),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(
                S::min_push_box(grid.iter().map(|row| row.chars().collect()).collect()),
                expected,
            );
        }
    }
}
