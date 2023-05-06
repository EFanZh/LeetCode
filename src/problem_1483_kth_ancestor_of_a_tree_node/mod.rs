pub mod binary_lifting;

pub trait TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self;
    fn get_kth_ancestor(&mut self, node: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::TreeAncestor;

    pub fn run<T: TreeAncestor>() {
        let test_cases = [
            (
                (7, &[-1, 0, 0, 1, 1, 2, 2] as &[_]),
                &[((3, 1), 1), ((5, 2), 0), ((6, 3), -1)] as &[_],
            ),
            (
                (5, &[-1, 0, 0, 0, 3]),
                &[((1, 5), -1), ((3, 2), -1), ((0, 1), -1), ((3, 1), 0), ((3, 5), -1)],
            ),
        ];

        for ((n, parent), operations) in test_cases {
            let mut tree_ancestor = T::new(n, parent.to_vec());

            for &((node, k), expected) in operations {
                assert_eq!(tree_ancestor.get_kth_ancestor(node, k), expected);
            }
        }
    }
}
