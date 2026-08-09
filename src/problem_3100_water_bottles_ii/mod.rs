pub mod mathematical;

pub trait Solution {
    fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((13, 6), 15), ((10, 3), 13)];

        for ((num_bottles, num_exchange), expected) in test_cases {
            assert_eq!(S::max_bottles_drunk(num_bottles, num_exchange), expected);
        }
    }
}
