pub mod iterative;

pub trait Solution {
    fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 5, 10] as &[_], 2), 2),
            ((&[3, 6, 8, 10], 3), 0),
            ((&[3, 4, 6, 7], 2), 1),
        ];

        for ((rungs, dist), expected) in test_cases {
            assert_eq!(S::add_rungs(rungs.to_vec(), dist), expected);
        }
    }
}
