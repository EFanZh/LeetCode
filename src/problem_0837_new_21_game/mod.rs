pub mod dynamic_programming;
pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn new21_game(n: i32, k: i32, max_pts: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((10, 1, 10), 1.0),
            ((6, 1, 10), 0.6),
            ((21, 17, 10), 0.73278),
            ((0, 0, 2), 1.0),
            ((98, 33, 68), 0.99898),
            ((2, 2, 2), 0.75),
        ];

        for ((n, k, max_pts), expected) in test_cases {
            approx::assert_abs_diff_eq!(S::new21_game(n, k, max_pts), expected, epsilon = 1.0e-5);
        }
    }
}
