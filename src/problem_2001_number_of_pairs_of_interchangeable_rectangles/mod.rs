pub mod hash_map;
pub mod hash_map_with_double_bytes;

pub trait Solution {
    fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[4, 8], [3, 6], [10, 20], [15, 30]] as &[_], 6),
            (&[[4, 5], [7, 8]], 0),
        ];

        for (rectangles, expected) in test_cases {
            assert_eq!(
                S::interchangeable_rectangles(rectangles.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
