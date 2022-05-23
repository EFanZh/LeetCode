pub mod sliding_window;

pub trait Solution {
    fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 0, 1, 2, 1, 1, 7, 5] as &[_], &[0, 1, 0, 1, 0, 1, 0, 1] as &[_], 3),
                16,
            ),
            ((&[1], &[0], 1), 1),
        ];

        for ((customers, grumpy, minutes), expected) in test_cases {
            assert_eq!(S::max_satisfied(customers.to_vec(), grumpy.to_vec(), minutes), expected);
        }
    }
}
