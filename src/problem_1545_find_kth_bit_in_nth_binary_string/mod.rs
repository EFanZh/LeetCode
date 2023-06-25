pub mod iterative;
pub mod mathematical;

pub trait Solution {
    fn find_kth_bit(n: i32, k: i32) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 1), '0'), ((3, 5), '0'), ((4, 11), '1'), ((4, 12), '0')];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::find_kth_bit(n, k), expected);
        }
    }
}
