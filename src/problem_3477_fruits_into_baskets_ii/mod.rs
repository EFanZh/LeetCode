pub mod iterative;

pub trait Solution {
    fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 2, 5] as &[_], &[5, 3, 4] as &[_]), 1),
            ((&[3, 6, 1], &[6, 4, 7]), 0),
        ];

        for ((fruits, baskets), expected) in test_cases {
            assert_eq!(S::num_of_unplaced_fruits(fruits.to_vec(), baskets.to_vec()), expected);
        }
    }
}
