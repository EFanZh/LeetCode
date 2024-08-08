pub mod greedy;

pub trait Solution {
    fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 4, 3] as &[_], &[2, 3, 1] as &[_]), 9),
            ((&[1, 2, 3, 2], &[2, 1, 2, 1]), 9),
            ((&[1], &[1]), 2),
        ];

        for ((plant_time, grow_time), expected) in test_cases {
            assert_eq!(
                S::earliest_full_bloom(plant_time.to_vec(), grow_time.to_vec()),
                expected,
            );
        }
    }
}
