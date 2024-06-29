pub mod greedy;

pub trait Solution {
    fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((10, &[3, 9, 19, 5, 21] as &[_]), true),
            ((5, &[4, 9, 23, 4]), false),
            ((i32::MAX, &[i32::MAX, i32::MAX, i32::MAX, i32::MAX]), true),
        ];

        for ((mass, asteroids), expected) in test_cases {
            assert_eq!(S::asteroids_destroyed(mass, asteroids.to_vec()), expected);
        }
    }
}
