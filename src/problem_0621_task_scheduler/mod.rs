pub mod greedy;
pub mod mathematical;

pub trait Solution {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("AAABBB", 2), 8), (("AAABBB", 0), 6), (("AAAAAABCDEFG", 2), 16)];

        for ((tasks, n), expected) in test_cases.iter().copied() {
            assert_eq!(S::least_interval(tasks.chars().collect(), n), expected);
        }
    }
}
