pub mod iterative;

pub trait Solution {
    fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::mem;

    pub fn run<S: Solution>() {
        let test_cases = [&[3, 3, 3, 3, 3, 1, 3] as &[_], &[2, 1, 3, 3, 3, 2]];

        let mut visited = Vec::new();

        for group_sizes in test_cases {
            visited.resize(group_sizes.len(), false);

            for group in S::group_the_people(group_sizes.to_vec()) {
                let group_size = group.len() as _;

                for value in group {
                    let value = value as usize;

                    assert!(!mem::replace(&mut visited[value], true));
                    assert_eq!(group_sizes[value], group_size);
                }
            }

            assert!(visited.iter().all(|&value| value));

            visited.clear();
        }
    }
}
