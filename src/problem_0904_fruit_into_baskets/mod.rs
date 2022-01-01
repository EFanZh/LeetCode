pub mod iterative;

pub trait Solution {
    fn total_fruit(fruits: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 1] as &[_], 3),
            (&[0, 1, 2, 2], 3),
            (&[1, 2, 3, 2, 2], 4),
            (&[3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4], 5),
            (&[0, 1, 2], 2),
            (&[0], 1),
            (&[0, 1, 6, 6, 4, 4, 6], 5),
        ];

        for (fruits, expected) in test_cases {
            assert_eq!(S::total_fruit(fruits.to_vec()), expected);
        }
    }
}
