pub mod binary_search;

pub trait Solution {
    fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 2] as &[_], 6.0), 1),
            ((&[1, 3, 2], 2.7), 3),
            ((&[1, 3, 2], 1.9), -1),
        ];

        for ((dist, hour), expected) in test_cases {
            assert_eq!(S::min_speed_on_time(dist.to_vec(), hour), expected);
        }
    }
}
