pub mod greedy;

pub trait Solution {
    fn broken_calc(start_value: i32, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 3), 2), ((5, 8), 2), ((3, 10), 3)];

        for ((start_value, target), expected) in test_cases {
            assert_eq!(S::broken_calc(start_value, target), expected);
        }
    }
}
