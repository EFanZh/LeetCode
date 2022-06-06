pub mod brute_force;

pub trait Solution {
    fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((7, 4), &[1, 2, 3, 1] as &[_]), ((10, 3), &[5, 2, 3])];

        for ((candies, num_people), expected) in test_cases {
            assert_eq!(S::distribute_candies(candies, num_people), expected);
        }
    }
}
