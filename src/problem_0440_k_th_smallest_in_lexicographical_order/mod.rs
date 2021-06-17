pub mod tree_preorder_traversal;

pub trait Solution {
    fn find_kth_number(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((10, 2), 10),
            ((4_897_764, 3_510_563), 4_159_503),
            ((10000, 10000), 9999),
            ((9, 4), 4),
            ((100, 10), 17),
            ((10, 3), 2),
            ((17, 1), 1),
            ((4_897_764, 4_330_851), 4_897_764),
        ];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::find_kth_number(n, k), expected);
        }
    }
}
