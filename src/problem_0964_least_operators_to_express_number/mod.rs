pub mod dynamic_programming;

pub trait Solution {
    fn least_ops_express_target(x: i32, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 19), 5), ((5, 501), 8), ((100, 100_000_000), 3)];

        for ((x, target), expected) in test_cases {
            assert_eq!(S::least_ops_express_target(x, target), expected);
        }
    }
}
