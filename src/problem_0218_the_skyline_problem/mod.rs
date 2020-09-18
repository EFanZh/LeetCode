pub mod divide_and_conquer;
pub mod ordered_map;
pub mod priority_queue;

pub trait Solution {
    fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[2, 9, 10], [3, 7, 15], [5, 12, 12], [15, 20, 10], [19, 24, 8]] as &[_],
                &[[2, 10], [3, 15], [7, 12], [12, 0], [15, 10], [20, 8], [24, 0]] as &[_],
            ),
            (&[[1, 2, 1], [1, 2, 2], [1, 2, 3]], &[[1, 3], [2, 0]]),
        ];

        for (building, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::get_skyline(building.iter().map(|building| building.to_vec()).collect()),
                expected
            );
        }
    }
}
