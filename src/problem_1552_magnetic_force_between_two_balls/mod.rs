pub mod binary_search;

pub trait Solution {
    fn max_distance(position: Vec<i32>, m: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 7] as &[_], 3), 3),
            ((&[5, 4, 3, 2, 1, 1_000_000_000], 2), 999_999_999),
            ((&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 3),
            ((&[79, 74, 57, 22], 4), 5),
        ];

        for ((position, m), expected) in test_cases {
            assert_eq!(S::max_distance(position.to_vec(), m), expected);
        }
    }
}
