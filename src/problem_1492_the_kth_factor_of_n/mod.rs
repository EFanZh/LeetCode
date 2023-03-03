pub mod iterative;

pub trait Solution {
    fn kth_factor(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((12, 3), 3),
            ((7, 2), 7),
            ((4, 4), -1),
            ((1000, 3), 4),
            ((2, 2), 2),
            ((12, 4), 4),
        ];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::kth_factor(n, k), expected);
        }
    }
}
