pub mod binary_search;

pub trait Solution {
    fn max_possible_score(start: Vec<i32>, d: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[6, 0, 3] as &[_], 2), 4), ((&[2, 6, 13, 13], 5), 5)];

        for ((start, d), expected) in test_cases {
            assert_eq!(S::max_possible_score(start.to_vec(), d), expected);
        }
    }
}
