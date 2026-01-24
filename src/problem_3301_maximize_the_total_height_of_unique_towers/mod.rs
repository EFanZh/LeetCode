pub mod greedy;

pub trait Solution {
    fn maximum_total_sum(maximum_height: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 4, 3] as &[_], 10), (&[15, 10], 25), (&[2, 2, 1], -1)];

        for (maximum_height, expected) in test_cases {
            assert_eq!(S::maximum_total_sum(maximum_height.to_vec()), expected);
        }
    }
}
