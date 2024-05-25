pub mod newtons_method;

pub trait Solution {
    fn minimum_perimeter(num: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 8),
            (13, 16),
            (1_000_000_000, 5040),
            (2_784_381_467_700, 70896),
            (613_724_426_267, 42824),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::minimum_perimeter(num), expected);
        }
    }
}
