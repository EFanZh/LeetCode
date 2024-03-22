pub mod obvious;

pub trait Solution {
    fn sum(num1: i32, num2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((12, 5), 17), ((-10, 4), -6)];

        for ((num1, num2), expected) in test_cases {
            assert_eq!(S::sum(num1, num2), expected);
        }
    }
}
