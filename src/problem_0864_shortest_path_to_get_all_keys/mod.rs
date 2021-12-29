pub mod bfs;

pub trait Solution {
    fn shortest_path_all_keys(grid: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["@.a.#", "###.#", "b.A.B"] as &[_], 8),
            (&["@..aA", "..B#.", "....b"], 6),
            (&["@Aa"], -1),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(
                S::shortest_path_all_keys(grid.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
