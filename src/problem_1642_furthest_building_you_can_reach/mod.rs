pub mod binary_heap;

pub trait Solution {
    fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 12, 2, 7, 3, 18, 20, 3, 19] as &[_], 10, 2), 7),
            ((&[14, 3, 19, 3], 17, 0), 3),
        ];

        for ((heights, bricks, ladders), expected) in test_cases {
            assert_eq!(S::furthest_building(heights.to_vec(), bricks, ladders), expected);
        }
    }
}
