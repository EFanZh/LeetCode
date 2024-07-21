pub mod dynamic_programming;

pub trait Solution {
    fn max_students(seats: Vec<Vec<char>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["#.##.#", ".####.", "#.##.#"] as &[_], 4),
            (&[".#", "##", "#.", "##", ".#"], 3),
            (&["#...#", ".#.#.", "..#..", ".#.#.", "#...#"], 10),
            (
                &[
                    "....#...", "........", "........", "......#.", "........", "..#.....", "........", "...#..#.",
                ],
                31,
            ),
        ];

        for (seats, expected) in test_cases {
            assert_eq!(
                S::max_students(seats.iter().map(|row| row.chars().collect()).collect()),
                expected,
            );
        }
    }
}
