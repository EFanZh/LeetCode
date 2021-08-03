pub mod iterative;

pub trait Solution {
    fn is_one_bit_character(bits: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 0, 0] as &[_], true), (&[1, 1, 1, 0], false)];

        for (bits, expected) in test_cases {
            assert_eq!(S::is_one_bit_character(bits.to_vec()), expected);
        }
    }
}
