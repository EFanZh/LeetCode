pub mod dynamic_programming;

pub trait Solution {
    fn maximum_total_damage(power: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 3, 4] as &[_], 6),
            (&[7, 1, 6, 6], 13),
            (&[5, 9, 2, 10, 2, 7, 10, 9, 3, 8], 31),
        ];

        for (power, expected) in test_cases {
            assert_eq!(S::maximum_total_damage(power.to_vec()), expected);
        }
    }
}
