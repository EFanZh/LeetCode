pub mod sweep_line;

pub trait Solution {
    fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((10, &[[5, 7], [1, 3], [9, 10]] as &[_]), 2),
            ((5, &[[2, 4], [1, 3]]), 1),
            ((6, &[[1, 6]]), 0),
            ((8, &[[3, 4], [4, 8], [2, 5], [3, 8]]), 1),
        ];

        for ((days, meetings), expected) in test_cases {
            assert_eq!(S::count_days(days, meetings.iter().map(Vec::from).collect()), expected,);
        }
    }
}
