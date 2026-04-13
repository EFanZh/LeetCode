pub mod greedy;

pub trait Solution {
    fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]] as &[_]), 1),
            ((3, &[[1, 1], [1, 2], [2, 1], [2, 2]]), 0),
            ((5, &[[1, 3], [3, 2], [3, 3], [3, 5], [5, 3]]), 1),
            ((3, &[[1, 2], [3, 1], [1, 1], [2, 3], [2, 2], [3, 2]]), 0),
        ];

        for ((n, buildings), expected) in test_cases {
            assert_eq!(
                S::count_covered_buildings(n, buildings.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
