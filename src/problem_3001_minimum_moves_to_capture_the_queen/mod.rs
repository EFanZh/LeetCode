pub mod iterative;

pub trait Solution {
    fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 1, 8, 8, 2, 3), 2), ((5, 3, 3, 4, 5, 2), 1)];

        for ((a, b, c, d, e, f), expected) in test_cases {
            assert_eq!(S::min_moves_to_capture_the_queen(a, b, c, d, e, f), expected,);
        }
    }
}
