pub mod iterative;

pub trait Solution {
    fn num_friend_requests(ages: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[16, 16] as &[_], 2),
            (&[16, 17, 18], 2),
            (&[20, 30, 100, 110, 120], 3),
            (&[8, 85, 24, 85, 69], 4),
        ];

        for (ages, expected) in test_cases {
            assert_eq!(S::num_friend_requests(ages.to_vec()), expected);
        }
    }
}
