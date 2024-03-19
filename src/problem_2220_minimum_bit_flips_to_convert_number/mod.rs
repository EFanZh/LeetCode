pub mod xor;

pub trait Solution {
    fn min_bit_flips(start: i32, goal: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((10, 7), 3), ((3, 4), 3)];

        for ((start, goal), expected) in test_cases {
            assert_eq!(S::min_bit_flips(start, goal), expected);
        }
    }
}
