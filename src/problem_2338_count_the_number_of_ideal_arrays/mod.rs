pub mod dynamic_programming_and_combinations;

pub trait Solution {
    fn ideal_arrays(n: i32, max_value: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 5), 10), ((5, 3), 11), ((5, 9), 111)];

        for ((n, max_value), expected) in test_cases {
            assert_eq!(S::ideal_arrays(n, max_value), expected);
        }
    }
}
