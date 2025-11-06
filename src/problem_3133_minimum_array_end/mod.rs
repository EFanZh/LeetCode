pub mod bit_manipulation;

pub trait Solution {
    fn min_end(n: i32, x: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 4), 6), ((2, 7), 15), ((2, 2), 3)];

        for ((n, x), expected) in test_cases {
            assert_eq!(S::min_end(n, x), expected);
        }
    }
}
