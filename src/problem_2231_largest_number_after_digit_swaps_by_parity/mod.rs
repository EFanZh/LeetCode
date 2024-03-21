pub mod iterative;

pub trait Solution {
    fn largest_integer(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1234, 3412), (65875, 87655)];

        for (num, expected) in test_cases {
            assert_eq!(S::largest_integer(num), expected);
        }
    }
}
