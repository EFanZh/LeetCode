pub mod iterative;

pub trait Solution {
    fn find_min_difference(time_points: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&["23:59", "00:00"] as &[_], 1), (&["00:00", "23:59", "00:00"], 0)];

        for (time_points, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_min_difference(time_points.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
