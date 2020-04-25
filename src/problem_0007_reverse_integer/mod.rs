pub mod brute_force;

pub trait Solution {
    fn reverse(x: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(123, 321), (-123, -321), (120, 21), (1_534_236_469, 0)];

        for (x, expected) in test_cases.iter().copied() {
            assert_eq!(S::reverse(x), expected);
        }
    }
}
