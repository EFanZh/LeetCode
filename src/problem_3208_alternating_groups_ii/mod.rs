pub mod iterative;

pub trait Solution {
    fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[0, 1, 0, 1, 0] as &[_], 3), 3),
            ((&[0, 1, 0, 0, 1, 0, 1], 6), 2),
            ((&[1, 1, 0, 1], 4), 0),
        ];

        for ((colors, k), expected) in test_cases {
            assert_eq!(S::number_of_alternating_groups(colors.to_vec(), k), expected);
        }
    }
}
