pub mod two_pointers;

pub trait Solution {
    fn max_area(height: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 8, 6, 2, 5, 4, 8, 3, 7] as &[_], 49),
            (&[1, 1], 1),
            (&[4, 3, 2, 1, 4], 16),
            (&[1, 2, 1], 2),
        ];

        for (height, expected) in test_cases.iter().copied() {
            assert_eq!(S::max_area(height.to_vec()), expected);
        }
    }
}
