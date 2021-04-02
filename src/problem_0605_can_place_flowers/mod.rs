pub mod iterative;

pub trait Solution {
    fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 0, 0, 0, 1] as &[_], 1), true),
            ((&[1, 0, 0, 0, 1], 2), false),
            ((&[0, 0, 1, 0, 1], 1), true),
            ((&[1, 0, 0, 0, 1, 0, 0], 2), true),
            ((&[1, 0, 0, 0, 1], 0), true),
        ];

        for ((flowerbed, n), expected) in test_cases.iter().copied() {
            assert_eq!(S::can_place_flowers(flowerbed.to_vec(), n), expected);
        }
    }
}
