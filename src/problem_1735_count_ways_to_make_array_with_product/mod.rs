pub mod combinations;

pub trait Solution {
    fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2, 6], [5, 1], [73, 660]] as &[_], &[4, 1, 50_734_910] as &[_]),
            (&[[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]], &[1, 2, 3, 10, 5]),
            (
                &[[373, 196], [101, 229], [466, 109], [308, 83], [296, 432]],
                &[865_201_973, 101, 466, 308, 411_805_778],
            ),
            (
                &[[474, 282], [38, 139], [235, 321], [383, 325], [224, 70]],
                &[106_496_424, 38, 55_225, 28_164_288, 11_239_424],
            ),
        ];

        for &(queries, expected) in &test_cases[3..] {
            assert_eq!(
                S::ways_to_fill_array(queries.iter().copied().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
