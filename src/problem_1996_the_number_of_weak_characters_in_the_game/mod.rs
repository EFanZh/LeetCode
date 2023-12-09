pub mod iterative;

pub trait Solution {
    fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[5, 5], [6, 3], [3, 6]] as &[_], 0),
            (&[[2, 2], [3, 3]], 1),
            (&[[1, 5], [10, 4], [4, 3]], 1),
        ];

        for (properties, expected) in test_cases {
            assert_eq!(
                S::number_of_weak_characters(properties.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
