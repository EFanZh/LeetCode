pub mod hash_map;

pub trait Solution {
    fn minimum_rounds(tasks: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 2, 3, 3, 2, 4, 4, 4, 4, 4] as &[_], 4), (&[2, 3, 3], -1)];

        for (tasks, expected) in test_cases {
            assert_eq!(S::minimum_rounds(tasks.to_vec()), expected);
        }
    }
}
