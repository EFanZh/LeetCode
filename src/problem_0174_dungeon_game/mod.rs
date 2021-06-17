pub mod dynamic_programming;

pub trait Solution {
    fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[-2, -3, 3] as &[_], &[-5, -10, 1], &[10, 30, -5]] as &[&[_]], 7),
            (&[&[1, -2, 3], &[2, -2, -2]], 2),
        ];

        for (dungeon, expected) in test_cases {
            assert_eq!(
                S::calculate_minimum_hp(dungeon.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
