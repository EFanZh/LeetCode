pub mod sliding_window;

pub trait Solution {
    fn max_score(card_points: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 6, 1] as &[_], 3), 12),
            ((&[2, 2, 2], 2), 4),
            ((&[9, 7, 7, 9, 7, 7, 9], 7), 55),
        ];

        for ((card_points, k), expected) in test_cases {
            assert_eq!(S::max_score(card_points.to_vec(), k), expected);
        }
    }
}
