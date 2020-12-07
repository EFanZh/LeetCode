pub mod bit_manipulation;

pub trait Solution {
    fn find_complement(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 0), (5, 2)];

        for (num, expected) in test_cases.iter().copied() {
            assert_eq!(S::find_complement(num), expected);
        }
    }
}
