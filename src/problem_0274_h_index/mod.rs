pub mod counting_sort;

pub trait Solution {
    fn h_index(citations: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 0, 6, 1, 5] as &[_], 3), (&[1, 3, 1], 1), (&[0], 0)];

        for (citations, expected) in test_cases {
            assert_eq!(S::h_index(citations.to_vec()), expected);
        }
    }
}
