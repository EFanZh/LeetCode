pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        match (n, presses) {
            (_, 0) => 1,
            (1, _) => 2,
            (2, 1) => 3,
            (2, _) | (_, 1) => 4,
            (_, 2) => 7,
            (_, _) => 8,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flip_lights(n: i32, presses: i32) -> i32 {
        Self::flip_lights(n, presses)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
