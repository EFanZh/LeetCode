pub mod hash_set;

pub trait Solution {
    fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 3, 1, 1), 8), ((2, 3, 1, 2), 5), ((200, 200, 10, 1), 764_262_396)];

        for ((low, high, zero, one), expected) in test_cases {
            assert_eq!(S::count_good_strings(low, high, zero, one), expected);
        }
    }
}
