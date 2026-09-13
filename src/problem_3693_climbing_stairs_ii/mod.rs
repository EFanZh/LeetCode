pub mod dynamic_programming;

pub trait Solution {
    fn climb_stairs(n: i32, costs: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[1, 2, 3, 4] as &[_]), 13),
            ((4, &[5, 1, 6, 2]), 11),
            ((3, &[9, 8, 3]), 12),
        ];

        for ((n, costs), expected) in test_cases {
            assert_eq!(S::climb_stairs(n, costs.to_vec()), expected);
        }
    }
}
