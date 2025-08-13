pub mod greedy;

pub trait Solution {
    fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[0, 1], [1, 2]] as &[_]), 0),
            ((4, &[[0, 2], [1, 3], [1, 2]]), -1),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(S::find_champion(n, edges.iter().map(Vec::from).collect()), expected,);
        }
    }
}
