pub mod bfs;

pub trait Solution {
    fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[&[1, 1, 1] as &[_], &[1, 1, 0], &[1, 0, 1]] as &[&[_]], 1, 1, 2),
                &[&[2, 2, 2] as &[_], &[2, 2, 0], &[2, 0, 1]] as &[&[_]],
            ),
            (
                (&[&[1, 1, 1], &[1, 1, 0], &[1, 0, 1]], 1, 1, 1),
                &[&[1, 1, 1], &[1, 1, 0], &[1, 0, 1]],
            ),
        ];

        for ((image, sr, sc, new_color), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::flood_fill(image.iter().map(|row| row.to_vec()).collect(), sr, sc, new_color),
                expected
            );
        }
    }
}
