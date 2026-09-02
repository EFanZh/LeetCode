pub mod iterative;

pub trait Solution {
    fn elevator_requests(n: i32, requests: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, &[2, 1, 4, 3] as &[_]), 7), ((3, &[2, 0, 0]), 4)];

        for ((n, requests), expected) in test_cases {
            assert_eq!(S::elevator_requests(n, requests.to_vec()), expected);
        }
    }
}
