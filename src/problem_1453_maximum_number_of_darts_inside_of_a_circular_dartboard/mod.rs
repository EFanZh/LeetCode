pub mod angular_sweep;

pub trait Solution {
    fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[-2, 0], [2, 0], [0, 2], [0, -2]] as &[_], 2), 4),
            ((&[[-3, 0], [3, 0], [2, 6], [5, 4], [0, 9], [7, 8]], 5), 5),
            ((&[[-2, 0], [2, 0], [0, 2], [0, -2]], 1), 1),
        ];

        for ((darts, r), expected) in test_cases {
            assert_eq!(S::num_points(darts.iter().map(Vec::from).collect(), r), expected);
        }
    }
}
