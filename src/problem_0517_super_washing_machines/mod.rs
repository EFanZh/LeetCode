pub mod iterative;

pub trait Solution {
    fn find_min_moves(machines: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 0, 5] as &[_], 3), (&[0, 3, 0], 2), (&[0, 2, 0], -1)];

        for (machines, expected) in test_cases {
            assert_eq!(S::find_min_moves(machines.to_vec()), expected);
        }
    }
}
