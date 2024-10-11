pub mod quick_select;

pub trait Solution {
    fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 4, 5] as &[_], &[1, 2, 4, 4] as &[_], 2), 3),
            ((&[10, 2, 2], &[2, 2, 0], 100), 3),
        ];

        for ((capacity, rocks, additional_rocks), expected) in test_cases {
            assert_eq!(
                S::maximum_bags(capacity.to_vec(), rocks.to_vec(), additional_rocks),
                expected,
            );
        }
    }
}
