pub mod mathematical;
pub mod mathematical_2;

pub trait Solution {
    fn count_odds(low: i32, high: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 7), 3), ((8, 10), 1)];

        for ((low, high), expected) in test_cases {
            assert_eq!(S::count_odds(low, high), expected);
        }
    }
}
