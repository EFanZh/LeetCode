pub mod sum_and_carry;

pub trait Solution {
    fn get_sum(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 2), 3), ((-2, 3), 1)];

        for ((a, b), expected) in test_cases.iter().copied() {
            assert_eq!(S::get_sum(a, b), expected);
        }
    }
}
