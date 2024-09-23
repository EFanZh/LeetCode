pub mod greedy;

pub trait Solution {
    fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 8], [6, 3], [8, 6]] as &[_], 5, 4), 9),
            ((&[[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]], 5, 4), 14),
            ((&[[0, 3], [6, 4], [8, 5]], 3, 2), 0),
        ];

        for ((fruits, start_pos, k), expected) in test_cases {
            assert_eq!(
                S::max_total_fruits(fruits.iter().map(Vec::from).collect(), start_pos, k),
                expected,
            );
        }
    }
}
