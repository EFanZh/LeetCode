pub mod iterative;

pub trait Solution {
    fn generate_key(num1: i32, num2: i32, num3: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 10, 1000), 0), ((987, 879, 798), 777), ((1, 2, 3), 1)];

        for ((num1, num2, num3), expected) in test_cases {
            assert_eq!(S::generate_key(num1, num2, num3), expected);
        }
    }
}
