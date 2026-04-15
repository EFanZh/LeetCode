pub mod iterative;

pub trait Solution {
    fn special_grid(n: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, &[&[0] as &[_]] as &[&[_]]),
            (1, &[&[3, 0], &[2, 1]]),
            (2, &[&[15, 12, 3, 0], &[14, 13, 2, 1], &[11, 8, 7, 4], &[10, 9, 6, 5]]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::special_grid(n), expected);
        }
    }
}
