pub mod newtons_method;
pub mod newtons_method_2;

pub trait Solution {
    fn minimum_perimeter(num: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 0),
            (1, 8),
            (2, 8),
            (3, 8),
            (4, 8),
            (5, 8),
            (6, 8),
            (7, 8),
            (8, 8),
            (9, 8),
            (10, 8),
            (11, 8),
            (12, 8),
            (13, 16),
            (60, 16),
            (168, 24),
            (360, 32),
            (660, 40),
            (1092, 48),
            (1_000_000_000, 5040),
            (2_784_381_467_700, 70896),
            (613_724_426_267, 42824),
            (1_000_000_000_000_000, 503_968),
            (6_148_912_027_310_325_423, 9_232_856),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::minimum_perimeter(num), expected);
        }
    }
}
