pub mod naive;

pub trait Solution {
    fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 1, 0] as &[_], &[1, 0, 1], &[0, 0, 0]] as &[&[_]],
                &[&[1, 0, 0] as &[_], &[0, 1, 0], &[1, 1, 1]] as &[&[_]],
            ),
            (
                &[&[1, 1, 0, 0], &[1, 0, 0, 1], &[0, 1, 1, 1], &[1, 0, 1, 0]],
                &[&[1, 1, 0, 0], &[0, 1, 1, 0], &[0, 0, 0, 1], &[1, 0, 1, 0]],
            ),
        ];

        for (a, expected) in test_cases {
            assert_eq!(
                S::flip_and_invert_image(a.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
