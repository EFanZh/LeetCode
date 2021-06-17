pub mod xor;

pub trait Solution {
    fn hamming_distance(x: i32, y: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 4), 2)];

        for ((x, y), expected) in test_cases {
            assert_eq!(S::hamming_distance(x, y), expected);
        }
    }
}
