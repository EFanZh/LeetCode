pub mod iterative;

pub trait Solution {
    fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 2, 3, 3] as &[_], 5), 14),
            ((&[1, 1, 1, 4, 2, 3], 4), 30),
            ((&[7, 7, 7, 7, 7, 7, 7], 8), 49),
        ];

        for ((plants, capacity), expected) in test_cases {
            assert_eq!(S::watering_plants(plants.to_vec(), capacity), expected);
        }
    }
}
