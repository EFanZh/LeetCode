pub mod two_pointers;
pub mod two_pointers_2;

pub trait Solution {
    fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2] as &[_], 3), 1),
            ((&[3, 2, 2, 1], 3), 3),
            ((&[3, 5, 3, 4], 5), 4),
        ];

        for ((people, limit), expected) in test_cases {
            assert_eq!(S::num_rescue_boats(people.to_vec(), limit), expected);
        }
    }
}
