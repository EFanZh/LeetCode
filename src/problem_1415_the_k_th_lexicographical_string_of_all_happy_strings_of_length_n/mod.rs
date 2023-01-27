pub mod mathematical;

pub trait Solution {
    fn get_happy_string(n: i32, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 3), "c"), ((1, 4), ""), ((3, 9), "cab"), ((10, 100), "abacbabacb")];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::get_happy_string(n, k), expected);
        }
    }
}
