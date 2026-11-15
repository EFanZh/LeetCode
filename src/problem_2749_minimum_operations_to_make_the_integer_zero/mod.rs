pub mod brute_force;

pub trait Solution {
    fn make_the_integer_zero(num1: i32, num2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, -2), 3), ((5, 7), -1)];

        for ((num1, num2), expected) in test_cases {
            assert_eq!(S::make_the_integer_zero(num1, num2), expected);
        }
    }
}
