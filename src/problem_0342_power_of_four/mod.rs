pub mod bit_mask;

pub trait Solution {
    fn is_power_of_four(num: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(16, true), (5, false)];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_power_of_four(num), expected);
        }
    }
}
