pub mod prefix_sums;

pub trait Solution {
    fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 5, 2, 4] as &[_], &[5, 1, 4, 2] as &[_]), 110),
            ((&[1, 1, 1], &[1, 1, 1]), 5),
            ((&[1, 2, 3, 4], &[1, 2]), 21),
        ];

        for ((skill, mana), expected) in test_cases {
            assert_eq!(S::min_time(skill.to_vec(), mana.to_vec()), expected);
        }
    }
}
