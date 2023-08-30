pub mod mathematical;

pub trait Solution {
    fn square_is_white(coordinates: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("a1", false), ("h3", true), ("c7", false)];

        for (coordinates, expected) in test_cases {
            assert_eq!(S::square_is_white(coordinates.to_string()), expected);
        }
    }
}
