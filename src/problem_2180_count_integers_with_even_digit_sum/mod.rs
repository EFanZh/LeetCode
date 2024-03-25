pub mod mathematical;

pub trait Solution {
    fn count_even(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(4, 2), (30, 14), (910, 455)];

        for (num, expected) in test_cases {
            assert_eq!(S::count_even(num), expected);
        }
    }
}
