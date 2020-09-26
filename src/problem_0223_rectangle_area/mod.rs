pub mod normalize;

pub trait Solution {
    #[allow(clippy::too_many_arguments)]
    fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((-3, 0, 3, 4, 0, -1, 9, 2), 45),
            ((0, 0, 0, 0, -1, -1, 1, 1), 4),
            ((-2, -2, 2, 2, 1, -3, 3, 3), 24),
        ];

        for ((a, b, c, d, e, f, g, h), expected) in test_cases.iter().copied() {
            assert_eq!(S::compute_area(a, b, c, d, e, f, g, h), expected);
        }
    }
}
