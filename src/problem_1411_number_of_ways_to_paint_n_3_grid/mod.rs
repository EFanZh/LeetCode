pub mod dynamic_programming;
pub mod matrix_multiplication;

pub trait Solution {
    fn num_of_ways(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 12), (5000, 30_228_214)];

        for (n, expected) in test_cases {
            assert_eq!(S::num_of_ways(n), expected);
        }
    }
}
