pub mod prefix_sum;

pub trait Solution {
    fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 5] as &[_], &[&[4, 8] as &[_], &[2, 8] as &[_]] as &[&[_]]), 6),
            ((&[2, 3, 5], &[&[1, 4], &[2, 3], &[3, 4]]), -1),
            ((&[3, 5, 8, 10, 11, 12], &[&[12], &[11, 9], &[10, 5, 14]]), 9),
        ];

        for ((packages, boxes), expected) in test_cases {
            assert_eq!(
                S::min_wasted_space(packages.to_vec(), boxes.iter().copied().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
