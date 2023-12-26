pub mod iterative;

pub trait Solution {
    fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 4, 3] as &[_], 4, 2), true),
            ((&[1, 5, 6], 3, 4), true),
            ((&[1, 2, 3, 4], 6, 4), false),
            ((&[3, 5, 3], 5, 3), false),
        ];

        for ((rolls, mean, n), has_solution) in test_cases {
            let result = S::missing_rolls(rolls.to_vec(), mean, n as _);

            if has_solution {
                assert_eq!(result.len(), n);

                for &value in rolls {
                    assert!(matches!(value, 1..=6));
                }

                assert_eq!(
                    mean * (rolls.len() + n) as i32,
                    rolls.iter().sum::<i32>() + result.iter().sum::<i32>(),
                );
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
