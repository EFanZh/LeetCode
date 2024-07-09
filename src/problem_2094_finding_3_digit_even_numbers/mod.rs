pub mod brute_force;
pub mod brute_force_2;

pub trait Solution {
    fn find_even_numbers(digits: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[2, 1, 3, 0] as &[_],
                &[102, 120, 130, 132, 210, 230, 302, 310, 312, 320] as &[_],
            ),
            (&[2, 2, 8, 8, 2], &[222, 228, 282, 288, 822, 828, 882]),
            (&[3, 7, 5], &[]),
        ];

        for (digits, expected) in test_cases {
            assert_eq!(S::find_even_numbers(digits.to_vec()), expected);
        }
    }
}
