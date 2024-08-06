pub mod binary_search;

pub trait Solution {
    fn next_beautiful_number(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 22), (1000, 1333), (3000, 3133)];

        for (n, expected) in test_cases {
            assert_eq!(S::next_beautiful_number(n), expected);
        }
    }
}
