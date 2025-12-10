pub mod mathematical;

pub trait Solution {
    fn max_height_of_triangle(red: i32, blue: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 4), 3), ((2, 1), 2), ((4, 9), 4)];

        for ((red, blue), expected) in test_cases {
            assert_eq!(S::max_height_of_triangle(red, blue), expected);
        }
    }
}
