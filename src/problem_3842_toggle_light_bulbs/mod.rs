pub mod iterative;

pub trait Solution {
    fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 30, 20, 10] as &[_], &[20, 30] as &[_]), (&[100, 100], &[])];

        for (bulbs, expected) in test_cases {
            assert_eq!(S::toggle_light_bulbs(bulbs.to_vec()), expected);
        }
    }
}
