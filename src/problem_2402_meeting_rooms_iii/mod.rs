pub mod binary_heap;

pub trait Solution {
    fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, &[[0, 10], [1, 5], [2, 7], [3, 4]] as &[_]), 0),
            ((3, &[[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]), 1),
            ((2, &[[0, 10], [1, 2], [12, 14], [13, 15]]), 0),
        ];

        for ((n, meetings), expected) in test_cases {
            assert_eq!(S::most_booked(n, meetings.iter().map(Vec::from).collect()), expected);
        }
    }
}
