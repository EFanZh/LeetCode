pub mod greedy;

pub trait Solution {
    fn max_area(coords: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 1], [1, 2], [3, 2], [3, 3]] as &[_], 2),
            (&[[1, 1], [2, 2], [3, 3]], -1),
        ];

        for (coords, expected) in test_cases {
            assert_eq!(S::max_area(coords.iter().map(Vec::from).collect()), expected);
        }
    }
}
