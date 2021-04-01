pub mod brute_force;

pub trait Solution {
    fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[&[1, 1, 1] as &[_], &[1, 0, 1], &[1, 1, 1]] as &[&[_]],
            &[&[0, 0, 0] as &[_], &[0, 0, 0], &[0, 0, 0]] as &[&[_]],
        )];

        for (m, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::image_smoother(m.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
