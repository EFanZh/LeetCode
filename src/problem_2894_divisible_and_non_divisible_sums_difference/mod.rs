pub mod mathematical;

pub trait Solution {
    fn difference_of_sums(n: i32, m: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((10, 3), 19), ((5, 6), 15), ((5, 1), -15)];

        for ((n, m), expected) in test_cases {
            assert_eq!(S::difference_of_sums(n, m), expected);
        }
    }
}
