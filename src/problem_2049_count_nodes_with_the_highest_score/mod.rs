pub mod recursive;

pub trait Solution {
    fn count_highest_score_nodes(parents: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[-1, 2, 0, 2, 0] as &[_], 3), (&[-1, 2, 0], 2), (&[-1, 0], 2)];

        for (parents, expected) in test_cases {
            assert_eq!(S::count_highest_score_nodes(parents.to_vec()), expected);
        }
    }
}
