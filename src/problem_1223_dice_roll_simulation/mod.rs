pub mod dynamic_programming;

pub trait Solution {
    fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, [1, 1, 2, 2, 2, 3]), 34),
            ((2, [1, 1, 1, 1, 1, 1]), 30),
            ((3, [1, 1, 1, 2, 2, 3]), 181),
            ((20, [8, 5, 10, 8, 7, 2]), 822_005_673),
        ];

        for ((n, roll_max), expected) in test_cases {
            assert_eq!(S::die_simulator(n, roll_max.to_vec()), expected);
        }
    }
}
