pub mod binary_search;
pub mod iterative;

pub trait Solution {
    fn h_index(citations: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 1, 3, 5, 6] as &[_], 3)];

        for (citations, expected) in test_cases.iter().copied() {
            assert_eq!(S::h_index(citations.to_vec()), expected);
        }
    }
}
