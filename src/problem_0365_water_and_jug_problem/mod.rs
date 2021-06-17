pub mod mathematical;

pub trait Solution {
    fn can_measure_water(x: i32, y: i32, z: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 5, 4), true), ((2, 6, 5), false), ((0, 0, 0), true)];

        for ((x, y, z), expected) in test_cases {
            assert_eq!(S::can_measure_water(x, y, z), expected);
        }
    }
}
