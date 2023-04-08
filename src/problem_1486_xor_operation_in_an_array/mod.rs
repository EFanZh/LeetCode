pub mod bit_manipulation;

pub trait Solution {
    fn xor_operation(n: i32, start: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 0), 8), ((4, 3), 8)];

        for ((n, start), expected) in test_cases {
            assert_eq!(S::xor_operation(n, start), expected);
        }
    }
}
