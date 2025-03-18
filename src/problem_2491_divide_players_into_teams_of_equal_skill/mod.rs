pub mod iterative;

pub trait Solution {
    fn divide_players(skill: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 2, 5, 1, 3, 4] as &[_], 22),
            (&[3, 4], 12),
            (&[1, 1, 2, 3], -1),
            (&[1000, 1000], 1_000_000),
            (&[2, 1, 5, 2], -1),
        ];

        for (skill, expected) in test_cases {
            assert_eq!(S::divide_players(skill.to_vec()), expected);
        }
    }
}
