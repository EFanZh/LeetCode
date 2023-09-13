pub mod iterative;

pub trait Solution {
    fn sum_base(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((34, 6), 9), ((10, 10), 1)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::sum_base(n, k), expected);
        }
    }
}
