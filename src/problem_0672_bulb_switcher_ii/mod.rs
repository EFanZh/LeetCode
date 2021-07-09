pub mod brute_force;
pub mod enumerate;

pub trait Solution {
    fn flip_lights(n: i32, presses: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 0), 1),
            ((1, 1), 2),
            ((1, 2), 2),
            ((1, 3), 2),
            ((1, 4), 2),
            ((2, 0), 1),
            ((2, 1), 3),
            ((2, 2), 4),
            ((2, 3), 4),
            ((2, 4), 4),
            ((3, 0), 1),
            ((3, 1), 4),
            ((3, 2), 7),
            ((3, 3), 8),
            ((3, 4), 8),
            ((4, 0), 1),
            ((4, 1), 4),
            ((4, 2), 7),
            ((4, 3), 8),
            ((4, 4), 8),
        ];

        for ((n, presses), expected) in test_cases {
            assert_eq!(S::flip_lights(n, presses), expected);
        }
    }
}
