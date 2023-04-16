pub mod binary_heap;

pub trait Solution {
    fn avoid_flood(rains: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], true),
            (&[1, 2, 0, 0, 2, 1], true),
            (&[1, 2, 0, 1, 2], false),
            (&[69, 0, 0, 0, 69], true),
            (&[1, 2, 0, 2, 3, 0, 1], true),
        ];

        let mut full_lakes = HashSet::new();

        for (rains, has_solution) in test_cases {
            let result = S::avoid_flood(rains.to_vec());

            if has_solution {
                assert_eq!(result.len(), rains.len());

                for (&value, &rain) in result.iter().zip(rains) {
                    if rain == 0 {
                        full_lakes.remove(&value);
                    } else {
                        assert_eq!(value, -1);
                        assert!(full_lakes.insert(rain));
                    }
                }

                full_lakes.clear();
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
