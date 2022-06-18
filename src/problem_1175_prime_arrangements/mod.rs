pub mod on_the_fly;
pub mod pre_computed;

pub trait Solution {
    fn num_prime_arrangements(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 4),
            (5, 12),
            (6, 36),
            (7, 144),
            (8, 576),
            (9, 2_880),
            (10, 17_280),
            (11, 86_400),
            (12, 604_800),
            (13, 3_628_800),
            (14, 29_030_400),
            (15, 261_273_600),
            (16, 612_735_986),
            (17, 289_151_874),
            (18, 180_670_593),
            (19, 445_364_737),
            (20, 344_376_809),
            (100, 682_289_015),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::num_prime_arrangements(n), expected);
        }
    }
}
