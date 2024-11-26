pub mod sliding_window;

pub trait Solution {
    fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((6, 2, 4), 5), ((4, 1, 3), 6), ((684, 18, 496), 653_668_527)];

        for ((n, delay, forget), expected) in test_cases {
            assert_eq!(S::people_aware_of_secret(n, delay, forget), expected);
        }
    }
}
