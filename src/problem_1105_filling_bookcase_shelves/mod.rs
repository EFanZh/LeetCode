pub mod dynamic_programming;

pub trait Solution {
    fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]] as &[_], 4),
                6,
            ),
            ((&[[1, 3], [2, 4], [3, 2]], 6), 4),
        ];

        for ((books, shelf_width), expected) in test_cases {
            assert_eq!(
                S::min_height_shelves(books.iter().map(Vec::from).collect(), shelf_width),
                expected
            );
        }
    }
}
