pub mod iterative;

pub trait Solution {
    fn find_winning_player(skills: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 2, 6, 3, 9] as &[_], 2), 2),
            ((&[2, 5, 4], 3), 1),
            ((&[11, 9, 12, 2, 20, 1, 8], 3), 4),
        ];

        for ((skills, k), expected) in test_cases {
            assert_eq!(S::find_winning_player(skills.to_vec(), k), expected);
        }
    }
}
