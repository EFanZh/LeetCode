pub mod stack;

pub trait Solution {
    fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 10, -5] as &[_], &[5, 10] as &[_]),
            (&[8, -8], &[]),
            (&[10, 2, -5], &[10]),
            (&[-2, -1, 1, 2], &[-2, -1, 1, 2]),
            (&[-2, -2, 1, -2], &[-2, -2, -2]),
            (&[-2, 1, -2, 1], &[-2, -2, 1]),
            (&[-2, -2, -2, -2], &[-2, -2, -2, -2]),
        ];

        for (asteroids, expected) in test_cases {
            assert_eq!(S::asteroid_collision(asteroids.to_vec()), expected);
        }
    }
}
