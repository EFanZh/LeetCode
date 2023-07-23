pub mod dynamic_programming;

pub trait Solution {
    fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 6, 8, 4] as &[_], 1, 3, 5), 4),
            ((&[4, 3, 1], 1, 0, 6), 5),
            ((&[5, 2, 1], 0, 2, 3), 0),
        ];

        for ((locations, start, finish, fuel), expected) in test_cases {
            assert_eq!(S::count_routes(locations.to_vec(), start, finish, fuel), expected);
        }
    }
}
