pub mod greedy;

pub trait Solution {
    fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 1, &[2, 3] as &[_], &[2] as &[_]), 4),
            ((1, 1, &[2], &[2]), 4),
            ((2, 3, &[2, 3], &[2, 4]), 4),
        ];

        for ((n, m, h_bars, v_bars), expected) in test_cases {
            assert_eq!(
                S::maximize_square_hole_area(n, m, h_bars.to_vec(), v_bars.to_vec()),
                expected,
            );
        }
    }
}
