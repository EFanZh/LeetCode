pub mod dfs;

pub trait Solution {
    fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[1, -1, 3, -1] as &[_], &[2, -1, -1, -1] as &[_]), true),
            ((4, &[1, -1, 3, -1], &[2, 3, -1, -1]), false),
            ((2, &[1, 0], &[-1, -1]), false),
            ((3, &[1, -1, -1], &[-1, -1, 1]), false),
            ((4, &[3, -1, 1, -1], &[-1, -1, 0, -1]), true),
            ((6, &[1, -1, -1, 4, -1, -1], &[2, -1, -1, 5, -1, -1]), false),
        ];

        for ((n, left_child, right_child), expected) in test_cases {
            assert_eq!(
                S::validate_binary_tree_nodes(n, left_child.to_vec(), right_child.to_vec()),
                expected,
            );
        }
    }
}
