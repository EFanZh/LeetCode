pub mod greedy;

pub trait Solution {
    fn min_penalty(period: i32, lights: Vec<i32>, arrival_time: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((8, &[2, 3] as &[_], &[2, 5, 8, 11] as &[_]), 5),
            ((10, &[3, 6, 8], &[4, 9, 15]), 1),
            ((5, &[2], &[2, 3, 4, 5, 6]), 3),
        ];

        for ((period, lights, arrival_time), expected) in test_cases {
            assert_eq!(S::min_penalty(period, lights.to_vec(), arrival_time.to_vec()), expected);
        }
    }
}
