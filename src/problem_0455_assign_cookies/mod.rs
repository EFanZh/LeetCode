pub mod greedy;

pub trait Solution {
    fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], &[1, 1] as &[_]), 1),
            ((&[1, 2], &[1, 2, 3]), 2),
            ((&[10, 9, 8, 7], &[5, 6, 7, 8]), 2),
        ];

        for ((g, s), expected) in test_cases.iter().copied() {
            assert_eq!(S::find_content_children(g.to_vec(), s.to_vec()), expected);
        }
    }
}
