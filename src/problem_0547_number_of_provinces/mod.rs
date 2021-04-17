pub mod dynamic_programming;

pub trait Solution {
    fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[1, 1, 0] as &[_], &[1, 1, 0], &[0, 0, 1]] as &[&[_]], 2),
            (&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]], 3),
        ];

        for (is_connected, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_circle_num(is_connected.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
