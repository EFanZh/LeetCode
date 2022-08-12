pub mod iterative;

pub trait Solution {
    fn subtract_product_and_sum(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(234, 15), (4421, 21), (114, -2)];

        for (n, expected) in test_cases {
            assert_eq!(S::subtract_product_and_sum(n), expected);
        }
    }
}
