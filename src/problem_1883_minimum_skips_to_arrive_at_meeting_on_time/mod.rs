pub mod dynamic_programming;

pub trait Solution {
    fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 2] as &[_], 4, 2), 1),
            ((&[7, 3, 5, 5], 2, 10), 2),
            ((&[7, 3, 5, 5], 1, 10), -1),
        ];

        for ((dist, speed, hours_before), expected) in test_cases {
            assert_eq!(S::min_skips(dist.to_vec(), speed, hours_before), expected);
        }
    }
}
