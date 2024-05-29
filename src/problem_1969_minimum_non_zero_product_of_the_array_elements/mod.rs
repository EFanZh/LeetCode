pub mod mathematical;

pub trait Solution {
    fn min_non_zero_product(p: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (2, 6), (3, 1512), (33, 861_896_614)];

        for (p, expected) in test_cases {
            assert_eq!(S::min_non_zero_product(p), expected);
        }
    }
}
