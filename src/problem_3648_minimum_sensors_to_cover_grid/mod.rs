pub mod mathematical;

pub trait Solution {
    fn min_sensors(n: i32, m: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 5, 1), 4), ((2, 2, 2), 1)];

        for ((n, m, k), expected) in test_cases {
            assert_eq!(S::min_sensors(n, m, k), expected);
        }
    }
}
