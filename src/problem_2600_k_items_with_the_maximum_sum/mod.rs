pub mod modular_arithmetic;

pub trait Solution {
    fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 2, 0, 2), 2), ((3, 2, 0, 4), 3), ((6, 6, 6, 13), 5)];

        for ((num_ones, num_zeros, num_neg_ones, k), expected) in test_cases {
            assert_eq!(
                S::k_items_with_maximum_sum(num_ones, num_zeros, num_neg_ones, k),
                expected,
            );
        }
    }
}
