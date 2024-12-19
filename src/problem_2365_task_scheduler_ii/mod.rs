pub mod hash_map;

pub trait Solution {
    fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 1, 2, 3, 1] as &[_], 3), 9), ((&[5, 8, 8, 5], 2), 6)];

        for ((tasks, space), expected) in test_cases {
            assert_eq!(S::task_scheduler_ii(tasks.to_vec(), space), expected);
        }
    }
}
