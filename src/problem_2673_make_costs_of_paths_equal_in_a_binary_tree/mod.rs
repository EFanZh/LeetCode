pub mod dynamic_programming;

pub trait Solution {
    fn min_increments(n: i32, cost: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((7, &[1, 5, 2, 2, 3, 3, 1] as &[_]), 6), ((3, &[5, 3, 3] as &[_]), 0)];

        for ((n, cost), expected) in test_cases {
            assert_eq!(S::min_increments(n, cost.to_vec()), expected);
        }
    }
}
