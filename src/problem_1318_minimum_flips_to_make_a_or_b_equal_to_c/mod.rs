pub mod bit_manipulation;

pub trait Solution {
    fn min_flips(a: i32, b: i32, c: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 6, 5), 3), ((4, 2, 7), 1), ((1, 2, 3), 0), ((8, 3, 5), 3)];

        for ((a, b, c), expected) in test_cases {
            assert_eq!(S::min_flips(a, b, c), expected);
        }
    }
}
