pub mod recurse_on_half;

pub trait Solution {
    fn my_pow(x: f64, n: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2.0, 10), 1024.0), ((2.1, 3), 9.261), ((2.0, -2), 0.25)];

        for ((x, n), expected) in test_cases.iter().copied() {
            assert!(test_utilities::almose_equal(S::my_pow(x, n), expected));
        }
    }
}
