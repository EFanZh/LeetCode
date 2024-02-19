pub mod bfs;

pub trait Solution {
    fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["####F", "#C...", "M...."] as &[_], 1, 2), true),
            ((&["M.C...F"], 1, 4), true),
            ((&["M.C...F"], 1, 3), false),
            ((&["..#C", "...#", "..M.", "#F..", "...."], 2, 1), true),
            ((&["C...#", "...#F", "....#", "M...."], 2, 3), false),
            ((&[".....", "...C.", "...#.", "...#M", "F..#."], 1, 2), false),
            ((&["..F", "#C#", "#..", "#.M"], 1, 2), false),
        ];

        for ((grid, cat_jump, mouse_jump), expected) in test_cases {
            assert_eq!(
                S::can_mouse_win(grid.iter().copied().map(str::to_string).collect(), cat_jump, mouse_jump),
                expected,
            );
        }
    }
}
