pub mod mathematical;

pub trait Solution {
    fn pass_the_pillow(n: i32, time: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    pub fn run<S: Solution>() {
        let test_cases = [((4, 5), 2), ((3, 2), 3)];

        for ((n, time), expected) in test_cases {
            assert_eq!(S::pass_the_pillow(n, time), expected);
        }
    }
}
