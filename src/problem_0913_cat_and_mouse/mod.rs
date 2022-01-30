pub mod minimax_queue;
pub mod minimax_stack;

pub trait Solution {
    fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[2, 5] as &[_], &[3], &[0, 4, 5], &[1, 4, 5], &[2, 3], &[0, 2, 3]] as &[&[_]],
                0,
            ),
            (&[&[1, 3], &[0], &[3], &[0, 2]], 1),
            (&[&[2, 3], &[3, 4], &[0, 4], &[0, 1], &[1, 2]], 1),
            (
                &[
                    &[6],
                    &[4],
                    &[9],
                    &[5],
                    &[1, 5],
                    &[3, 4, 6],
                    &[0, 5, 10],
                    &[8, 9, 10],
                    &[7],
                    &[2, 7],
                    &[6, 7],
                ],
                1,
            ),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(
                S::cat_mouse_game(graph.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
