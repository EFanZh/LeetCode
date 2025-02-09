pub mod greedy;

pub trait Solution {
    fn minimize_xor(num1: i32, num2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 5), 3), ((1, 12), 3), ((25, 72), 24), ((8, 75), 15), ((91, 18), 80)];

        for ((num1, num2), expected) in test_cases {
            assert_eq!(S::minimize_xor(num1, num2), expected);
        }
    }
}
