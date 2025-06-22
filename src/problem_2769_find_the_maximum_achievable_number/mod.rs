pub mod mathematical;

pub trait Solution {
    fn the_maximum_achievable_x(num: i32, t: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((4, 1), 6), ((3, 2), 7)];

        for ((num, t), expected) in test_cases {
            assert_eq!(S::the_maximum_achievable_x(num, t), expected);
        }
    }
}
