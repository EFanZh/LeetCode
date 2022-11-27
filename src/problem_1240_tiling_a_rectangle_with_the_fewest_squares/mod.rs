pub mod backtracking;

pub trait Solution {
    fn tiling_rectangle(n: i32, m: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 3), 3), ((5, 8), 5), ((11, 13), 6)];

        for ((n, m), expected) in test_cases {
            assert_eq!(S::tiling_rectangle(n, m), expected);
        }
    }
}
