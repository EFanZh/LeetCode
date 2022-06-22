pub mod iterative;

pub trait Solution {
    fn path_in_zig_zag_tree(label: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(14, &[1, 3, 4, 14] as &[_]), (26, &[1, 2, 6, 10, 26]), (5, &[1, 3, 5])];

        for (label, expected) in test_cases {
            assert_eq!(S::path_in_zig_zag_tree(label), expected);
        }
    }
}
