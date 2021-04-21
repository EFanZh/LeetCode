pub mod hash_map;

pub trait Solution {
    fn least_bricks(wall: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[
                &[1, 2, 2, 1] as &[_],
                &[3, 1, 2],
                &[1, 3, 2],
                &[2, 4],
                &[3, 1, 2],
                &[1, 3, 1, 1],
            ] as &[&[_]],
            2,
        )];

        for (wall, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::least_bricks(wall.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
