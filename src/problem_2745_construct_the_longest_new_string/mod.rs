pub mod mathematical;

pub trait Solution {
    fn longest_string(x: i32, y: i32, z: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 5, 1), 12), ((3, 2, 2), 14)];

        for ((x, y, z), expected) in test_cases {
            assert_eq!(S::longest_string(x, y, z), expected);
        }
    }
}
