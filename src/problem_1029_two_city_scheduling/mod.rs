pub mod dynamic_programming;

pub trait Solution {
    fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[10, 20], [30, 200], [400, 50], [30, 20]] as &dyn Matrix<_>, 110),
            (
                &[[259, 770], [448, 54], [926, 667], [184, 139], [840, 118], [577, 469]],
                1859,
            ),
            (
                &[
                    [515, 563],
                    [451, 713],
                    [537, 709],
                    [343, 819],
                    [855, 779],
                    [457, 60],
                    [650, 359],
                    [631, 42],
                ],
                3086,
            ),
        ];

        for (costs, expected) in test_cases {
            assert_eq!(S::two_city_sched_cost(costs.to_vec()), expected);
        }
    }
}
