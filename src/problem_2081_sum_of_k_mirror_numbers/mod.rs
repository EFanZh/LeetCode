pub mod cheating;
pub mod cheating_2;

pub trait Solution {
    fn k_mirror(k: i32, n: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 5), 25), ((3, 7), 499), ((7, 17), 20_379_000)];

        for ((k, n), expected) in test_cases {
            assert_eq!(S::k_mirror(k, n), expected);
        }
    }
}
