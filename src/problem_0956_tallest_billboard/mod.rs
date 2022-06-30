pub mod dynamic_programming;

pub trait Solution {
    fn tallest_billboard(rods: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 6] as &[_], 6),
            (&[1, 2, 3, 4, 5, 6], 10),
            (&[1, 2], 0),
            (
                &[
                    124, 121, 107, 127, 156, 146, 135, 153, 137, 150, 141, 138, 129, 142, 124, 144, 126, 900, 900, 900,
                ],
                2500,
            ),
        ];

        for (rods, expected) in test_cases {
            assert_eq!(S::tallest_billboard(rods.to_vec()), expected);
        }
    }
}
