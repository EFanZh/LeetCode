pub mod binary_search;

pub trait Solution {
    fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], 5), 3),
            ((&[2], 1), 2),
            ((&[9, 3, 10, 5], 2), 5),
            ((&[1, 5], 4), 4),
        ];

        for ((time, total_trips), expected) in test_cases {
            assert_eq!(S::minimum_time(time.to_vec(), total_trips), expected);
        }
    }
}
