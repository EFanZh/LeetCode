pub mod mathematical;

pub trait Solution {
    fn check_good_integer(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1000, false), (19, true)];

        for (n, expected) in test_cases {
            assert_eq!(S::check_good_integer(n), expected);
        }
    }
}
