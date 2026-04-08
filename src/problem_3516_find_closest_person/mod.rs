pub mod mathematical;

pub trait Solution {
    fn find_closest(x: i32, y: i32, z: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 7, 4), 1), ((2, 5, 6), 2), ((1, 5, 3), 0)];

        for ((x, y, z), expected) in test_cases {
            assert_eq!(S::find_closest(x, y, z), expected);
        }
    }
}
