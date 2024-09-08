pub mod dynamic_programming;

pub trait Solution {
    fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 0] as &[_], 2), (&[0, 0, 2], 6), (&[0, 1, 2, 0], 6)];

        for (next_visit, expected) in test_cases {
            assert_eq!(S::first_day_been_in_all_rooms(next_visit.to_vec()), expected);
        }
    }
}
