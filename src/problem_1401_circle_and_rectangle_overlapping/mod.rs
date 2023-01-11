pub mod mathematical;

pub trait Solution {
    fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 0, 0, 1, -1, 3, 1), true),
            ((1, 1, 1, 1, -3, 2, -1), false),
            ((1, 0, 0, -1, 0, 0, 1), true),
            ((2, 1, 2, 5, 8, 6, 12), false),
            ((2, 3, 1, 0, 6, 1, 10), false),
            ((3, 4, 1, 3, 3, 8, 5), true),
            ((1, 1, 1, -3, -3, 3, 3), true),
            ((5, 8, 8, 9, 5, 12, 8), true),
            ((31, 23, 46, 12, 38, 15, 41), true),
        ];

        for ((radius, x_center, y_center, x1, y1, x2, y2), expected) in test_cases {
            assert_eq!(S::check_overlap(radius, x_center, y_center, x1, y1, x2, y2), expected);
        }
    }
}
