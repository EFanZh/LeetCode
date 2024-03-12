pub mod mathematical;

pub trait Solution {
    fn count_operations(num1: i32, num2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 3), 3), ((10, 10), 1), ((79, 68), 14)];

        for ((num1, num2), expected) in test_cases {
            assert_eq!(S::count_operations(num1, num2), expected);
        }
    }
}
