pub mod iterative;

pub struct CustomFunction(fn(i32, i32) -> i32);

impl CustomFunction {
    pub fn f(&self, x: i32, y: i32) -> i32 {
        (self.0)(x, y)
    }
}

pub trait Solution {
    fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::{CustomFunction, Solution};
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ((|x, y| x + y) as fn(_, _) -> _, 5),
                &[[1, 4], [2, 3], [3, 2], [4, 1]] as &[_],
            ),
            ((|x, y| x * y, 5), &[[1, 5], [5, 1]]),
        ];

        for ((customfunction, z), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_solution(&CustomFunction(customfunction), z)),
                expected,
            );
        }
    }
}
