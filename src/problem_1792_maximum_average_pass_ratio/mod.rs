pub mod greedy_binary_heap;

pub trait Solution {
    fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2], [3, 5], [2, 2]] as &[_], 2), 47.0 / 60.0),
            ((&[[2, 4], [3, 9], [4, 5], [2, 10]], 4), 353.0 / 660.0),
            (
                (&[[13609, 17094], [24079, 89827]], 22159),
                778_804_865.0 / 1_267_272_072.0,
            ),
        ];

        for ((classes, extra_students), expected) in test_cases {
            approx::assert_ulps_eq!(
                S::max_average_ratio(classes.iter().map(Vec::from).collect(), extra_students),
                expected,
            );
        }
    }
}
