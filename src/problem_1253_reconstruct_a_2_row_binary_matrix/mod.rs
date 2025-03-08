pub mod greedy;

pub trait Solution {
    fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 1, &[1, 1, 1] as &[_]), true),
            ((2, 3, &[2, 2, 1, 1]), false),
            ((5, 5, &[2, 1, 2, 0, 1, 0, 1, 2, 0, 1]), true),
            ((3, 0, &[1, 0, 2, 2, 1]), false),
        ];

        for ((upper, lower, colsum), has_solution) in test_cases {
            let result = S::reconstruct_matrix(upper, lower, colsum.to_vec());

            if has_solution {
                let [top, bottom] = result.try_into().unwrap();

                assert_eq!(top.len(), colsum.len());
                assert_eq!(bottom.len(), colsum.len());
                assert_eq!(top.iter().sum::<i32>(), upper);
                assert_eq!(bottom.iter().sum::<i32>(), lower);

                assert!(
                    top.into_iter()
                        .zip(bottom)
                        .map(|(top, bottom)| top + bottom)
                        .eq(colsum.iter().copied())
                );
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
