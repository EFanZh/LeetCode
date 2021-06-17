pub mod iterative;

pub trait Solution {
    fn get_permutation(n: i32, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 3), "213"), ((4, 9), "2314"), ((1, 1), "1")];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::get_permutation(n, k), expected);
        }
    }
}
