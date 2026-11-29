pub mod greedy;

pub trait Solution {
    fn can_remove_all(types: Vec<i32>, colors: Vec<i32>, group_size: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[0, 1, 1, 0] as &[_], &[5, 3, 3, 5] as &[_], &[2, 2] as &[_]), true),
            ((&[0, 1, 0], &[9, 9, 9], &[3, 2]), false),
            ((&[2, 0, 1, 2], &[7, 5, 4, 7], &[3, 1, 2]), false),
        ];

        for ((types, colors, group_size), expected) in test_cases {
            assert_eq!(
                S::can_remove_all(types.to_vec(), colors.to_vec(), group_size.to_vec()),
                expected,
            );
        }
    }
}
