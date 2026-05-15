pub mod mathematical;

pub trait Solution {
    fn gcd_of_odd_even_sums(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(4, 4), (5, 5)];

        for (n, expected) in test_cases {
            assert_eq!(S::gcd_of_odd_even_sums(n), expected);
        }
    }
}
