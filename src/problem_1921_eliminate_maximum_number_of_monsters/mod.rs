pub mod buckets;

pub trait Solution {
    fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 4] as &[_], &[1, 1, 1] as &[_]), 3),
            ((&[1, 1, 2, 3], &[1, 1, 1, 1]), 1),
            ((&[3, 2, 4], &[5, 3, 2]), 1),
        ];

        for ((dist, speed), expected) in test_cases {
            assert_eq!(S::eliminate_maximum(dist.to_vec(), speed.to_vec()), expected);
        }
    }
}
