pub mod mathematical;
pub mod mathematical_2;

pub trait Solution {
    fn min_cost_to_move_chips(position: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], 1), (&[2, 2, 2, 3, 3], 2), (&[1, 1_000_000_000], 1)];

        for (position, expected) in test_cases {
            assert_eq!(S::min_cost_to_move_chips(position.to_vec()), expected);
        }
    }
}
