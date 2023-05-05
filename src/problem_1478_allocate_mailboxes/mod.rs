pub mod dynamic_programming;

pub trait Solution {
    fn min_distance(houses: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 4, 8, 10, 20] as &[_], 3), 5), ((&[2, 3, 5, 12, 18], 2), 9)];

        for ((houses, k), expected) in test_cases {
            assert_eq!(S::min_distance(houses.to_vec(), k), expected);
        }
    }
}
