pub mod find_major_element;

pub trait Solution {
    fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 1, 2, 4, 2, 2] as &[_], &[5, 2, 6, 2, 3, 2] as &[_]), 2),
            ((&[3, 5, 1, 2, 3], &[3, 6, 3, 3, 4]), -1),
            ((&[1, 3, 4, 1, 2, 1, 3, 4], &[1, 3, 1, 2, 2, 1, 4, 4]), -1),
        ];

        for ((tops, bottoms), expected) in test_cases {
            assert_eq!(S::min_domino_rotations(tops.to_vec(), bottoms.to_vec()), expected);
        }
    }
}
