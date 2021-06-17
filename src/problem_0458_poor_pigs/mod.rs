pub mod mathematical;

pub trait Solution {
    fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1000, 15, 60), 5), ((4, 15, 15), 2), ((4, 15, 30), 2)];

        for ((buckets, minutes_to_die, minutes_to_test), expected) in test_cases {
            assert_eq!(S::poor_pigs(buckets, minutes_to_die, minutes_to_test), expected);
        }
    }
}
